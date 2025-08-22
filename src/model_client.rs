use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use async_stream::stream;
use futures::{pin_mut, StreamExt};
use tonic::codegen::{Body, Bytes, StdError};

use crate::{
    client::{
        batch_check_single_result::CheckResult, BatchCheckItem, BatchCheckRequest, CheckRequest,
        CheckRequestTupleKey, ConsistencyPreference, ContextualTupleKeys, ExpandRequest,
        ExpandRequestTupleKey, ListObjectsRequest, ListObjectsResponse, OpenFgaServiceClient,
        ReadRequest, ReadRequestTupleKey, ReadResponse, Tuple, TupleKey, TupleKeyWithoutCondition,
        UsersetTree, WriteRequest, WriteRequestDeletes, WriteRequestWrites,
    },
    error::{Error, Result},
};

const DEFAULT_MAX_TUPLES_PER_WRITE: i32 = 100;

#[derive(Clone, Debug)]
/// Wrapper around the generated [`OpenFgaServiceClient`].
///
/// Why you should use this wrapper:
///
/// * Handles the `store_id` and `authorization_model_id` for you - you don't need to pass them in every request
/// * Applies the same configured `consistency` to all requests
/// * Ensures the number of writes and deletes does not exceed OpenFGA's limit
/// * Uses tracing to log errors
/// * Never sends empty writes or deletes, which fails on OpenFGA
/// * Uses `impl Into<ReadRequestTupleKey>` arguments instead of very specific types like [`ReadRequestTupleKey`]
/// * Most methods don't require mutable access to the client. Cloning tonic clients is cheap.
/// * If a method is missing, the [`OpenFgaClient::client()`] provides access to the underlying client with full control
///
/// # Example
///
/// ```no_run
/// use openfga_client::client::{OpenFgaServiceClient, OpenFgaClient};
/// use tonic::transport::Channel;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let endpoint = "http://localhost:8080";
///     let service_client = OpenFgaServiceClient::connect(endpoint).await?;
///     let client = OpenFgaClient::new(service_client, "<store_id>", "<authorization_model_id>");
///
///     // Use the client to interact with OpenFGA
///     Ok(())
/// }
/// ```
pub struct OpenFgaClient<T> {
    client: OpenFgaServiceClient<T>,
    inner: Arc<ModelClientInner>,
}

#[derive(Debug, Clone)]
struct ModelClientInner {
    store_id: String,
    authorization_model_id: String,
    max_tuples_per_write: i32,
    consistency: ConsistencyPreference,
}

#[cfg(feature = "auth-middle")]
/// Specialization of the [`OpenFgaClient`] that includes optional
/// authentication with pre-shared keys (Bearer tokens) or client credentials.
/// For more fine-granular control, construct [`OpenFgaClient`] directly
/// with a custom [`OpenFgaServiceClient`].
pub type BasicOpenFgaClient = OpenFgaClient<crate::client::BasicAuthLayer>;

impl<T> OpenFgaClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    /// Create a new `OpenFgaModelClient` with the given `store_id` and `authorization_model_id`.
    #[must_use]
    pub fn new(
        client: OpenFgaServiceClient<T>,
        store_id: &str,
        authorization_model_id: &str,
    ) -> Self {
        OpenFgaClient {
            client,
            inner: Arc::new(ModelClientInner {
                store_id: store_id.to_string(),
                authorization_model_id: authorization_model_id.to_string(),
                max_tuples_per_write: DEFAULT_MAX_TUPLES_PER_WRITE,
                consistency: ConsistencyPreference::MinimizeLatency,
            }),
        }
    }

    /// Set the `max_tuples_per_write` for the client.
    #[must_use]
    pub fn set_max_tuples_per_write(mut self, max_tuples_per_write: i32) -> Self {
        let inner = Arc::unwrap_or_clone(self.inner);
        self.inner = Arc::new(ModelClientInner {
            store_id: inner.store_id,
            authorization_model_id: inner.authorization_model_id,
            max_tuples_per_write,
            consistency: inner.consistency,
        });
        self
    }

    /// Set the `consistency` for the client.
    #[must_use]
    pub fn set_consistency(mut self, consistency: impl Into<ConsistencyPreference>) -> Self {
        let inner = Arc::unwrap_or_clone(self.inner);
        self.inner = Arc::new(ModelClientInner {
            store_id: inner.store_id,
            authorization_model_id: inner.authorization_model_id,
            max_tuples_per_write: inner.max_tuples_per_write,
            consistency: consistency.into(),
        });
        self
    }

    /// Get the `store_id` of the client.
    pub fn store_id(&self) -> &str {
        &self.inner.store_id
    }

    /// Get the `authorization_model_id` of the client.
    pub fn authorization_model_id(&self) -> &str {
        &self.inner.authorization_model_id
    }

    /// Get the `max_tuples_per_write` of the client.
    pub fn max_tuples_per_write(&self) -> i32 {
        self.inner.max_tuples_per_write
    }

    /// Get the underlying `OpenFgaServiceClient`.
    pub fn client(&self) -> OpenFgaServiceClient<T> {
        self.client.clone()
    }

    /// Get the `consistency` of the client.
    pub fn consistency(&self) -> ConsistencyPreference {
        self.inner.consistency
    }

    /// Write or delete tuples from FGA.
    /// This is a wrapper around [`OpenFgaServiceClient::write`] that ensures that:
    ///
    /// * Ensures the number of writes and deletes does not exceed OpenFGA's limit
    /// * Does not send empty writes or deletes
    /// * Traces any errors that occur
    /// * Enriches the error with the `write_request` that caused the error
    ///
    /// All writes happen in a single transaction.
    ///
    /// OpenFGA currently has a default limit of 100 tuples per write
    /// (sum of writes and deletes).
    ///
    /// This `write` method will fail if the number of writes and deletes exceeds
    /// `max_tuples_per_write` which defaults to 100.
    /// To change this limit, use [`Self::set_max_tuples_per_write`].
    ///
    /// # Errors
    /// * [`Error::TooManyWrites`] if the number of writes and deletes exceeds `max_tuples_per_write`
    /// * [`Error::RequestFailed`] if the write request fails
    ///
    pub async fn write(
        &self,
        writes: impl Into<Option<Vec<TupleKey>>>,
        deletes: impl Into<Option<Vec<TupleKeyWithoutCondition>>>,
    ) -> Result<()> {
        let writes = writes.into().and_then(|w| (!w.is_empty()).then_some(w));
        let deletes = deletes.into().and_then(|d| (!d.is_empty()).then_some(d));

        if writes.is_none() && deletes.is_none() {
            return Ok(());
        }

        let num_writes_and_deletes = i32::try_from(
            writes
                .as_ref()
                .map_or(0, Vec::len)
                .checked_add(deletes.as_ref().map_or(0, Vec::len))
                .unwrap_or(usize::MAX),
        )
        .unwrap_or(i32::MAX);

        if num_writes_and_deletes > self.max_tuples_per_write() {
            tracing::error!(
                "Too many writes and deletes in single OpenFGA transaction (actual) {} > {} (max)",
                num_writes_and_deletes,
                self.max_tuples_per_write()
            );
            return Err(Error::TooManyWrites {
                actual: num_writes_and_deletes,
                max: self.max_tuples_per_write(),
            });
        }

        let write_request = WriteRequest {
            store_id: self.store_id().to_string(),
            writes: writes.map(|writes| WriteRequestWrites { tuple_keys: writes }),
            deletes: deletes.map(|deletes| WriteRequestDeletes {
                tuple_keys: deletes,
            }),
            authorization_model_id: self.authorization_model_id().to_string(),
        };

        self.client
            .clone()
            .write(write_request.clone())
            .await
            .map_err(|e| {
                let write_request_debug = format!("{write_request:?}");
                tracing::error!(
                    "Write request failed with status {e}. Request: {write_request_debug}"
                );
                Error::RequestFailed(e)
            })
            .map(|_| ())
    }

    /// Read tuples from OpenFGA.
    /// This is a wrapper around [`OpenFgaServiceClient::read`] that:
    ///
    /// * Traces any errors that occur
    /// * Enriches the error with the `read_request` that caused the error
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] if the read request fails
    pub async fn read(
        &self,
        page_size: i32,
        tuple_key: impl Into<ReadRequestTupleKey>,
        continuation_token: impl Into<Option<String>>,
    ) -> Result<tonic::Response<ReadResponse>> {
        let read_request = ReadRequest {
            store_id: self.store_id().to_string(),
            page_size: Some(page_size),
            continuation_token: continuation_token.into().unwrap_or_default(),
            tuple_key: Some(tuple_key.into()),
            consistency: self.consistency().into(),
        };
        self.client
            .clone()
            .read(read_request.clone())
            .await
            .map_err(|e| {
                let read_request_debug = format!("{read_request:?}");
                tracing::error!(
                    "Read request failed with status {e}. Request: {read_request_debug}"
                );
                Error::RequestFailed(e)
            })
    }

    /// Read all tuples, with pagination.
    /// For details on the parameters, see [`OpenFgaServiceClient::read_all_pages`].
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] If a request to OpenFGA fails.
    /// * [`Error::TooManyPages`] If the number of pages read exceeds `max_pages`.
    ///
    pub async fn read_all_pages(
        &self,
        tuple: Option<impl Into<ReadRequestTupleKey>>,
        page_size: i32,
        max_pages: u32,
    ) -> Result<Vec<Tuple>> {
        let store_id = self.store_id().to_string();
        self.client
            .clone()
            .read_all_pages(&store_id, tuple, self.consistency(), page_size, max_pages)
            .await
    }

    /// Perform a check.
    /// Returns `true` if the check is allowed, `false` otherwise.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] if the check request fails
    ///
    pub async fn check(
        &self,
        tuple_key: impl Into<CheckRequestTupleKey>,
        contextual_tuples: impl Into<Option<Vec<TupleKey>>>,
        context: impl Into<Option<prost_wkt_types::Struct>>,
        trace: bool,
    ) -> Result<bool> {
        let contextual_tuples = contextual_tuples
            .into()
            .and_then(|c| (!c.is_empty()).then_some(c))
            .map(|tuple_keys| ContextualTupleKeys { tuple_keys });

        let check_request = CheckRequest {
            store_id: self.store_id().to_string(),
            tuple_key: Some(tuple_key.into()),
            consistency: self.consistency().into(),
            contextual_tuples,
            authorization_model_id: self.authorization_model_id().to_string(),
            context: context.into(),
            trace,
        };
        let response = self
            .client
            .clone()
            .check(check_request.clone())
            .await
            .map_err(|e| {
                let check_request_debug = format!("{check_request:?}");
                tracing::error!(
                    "Check request failed with status {e}. Request: {check_request_debug}"
                );
                Error::RequestFailed(e)
            })?;
        Ok(response.get_ref().allowed)
    }

    /// Check multiple tuples at once.
    /// Returned `HashMap` contains one key for each `correlation_id` in the input.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] if the check request fails
    /// * [`Error::ExpectedOneof`] if the server unexpectedly returns `None` for one of the tuples
    ///   to check.
    pub async fn batch_check<I>(
        &self,
        checks: impl IntoIterator<Item = I>,
    ) -> Result<HashMap<String, CheckResult>>
    where
        I: Into<BatchCheckItem>,
    {
        let checks: Vec<BatchCheckItem> = checks.into_iter().map(Into::into).collect();
        let request = BatchCheckRequest {
            store_id: self.store_id().to_string(),
            checks,
            authorization_model_id: self.authorization_model_id().to_string(),
            consistency: self.consistency().into(),
        };

        let response = self
            .client
            .clone()
            .batch_check(request.clone())
            .await
            .map_err(|e| {
                let request_debug = format!("{request:?}");
                tracing::error!(
                    "Batch-Check request failed with status {e}. Request: {request_debug}"
                );
                Error::RequestFailed(e)
            })?;

        let mut map = HashMap::new();
        for (k, v) in response.into_inner().result {
            match v.check_result {
                // The server should return `Some(_)` for every tuple to check.
                // `None` is not expected to occur, hence returning an error for the *entire*
                // batch request to keep the API simple.
                Some(v) => map.insert(k, v),
                None => return Err(Error::ExpectedOneof),
            };
        }
        Ok(map)
    }

    /// Expand all relationships in userset tree format.
    /// Useful to reason about and debug a certain relationship.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] if the expand request fails
    ///
    pub async fn expand(
        &self,
        tuple_key: impl Into<ExpandRequestTupleKey>,
        contextual_tuples: impl Into<Option<Vec<TupleKey>>>,
    ) -> Result<Option<UsersetTree>> {
        let expand_request = ExpandRequest {
            store_id: self.store_id().to_string(),
            tuple_key: Some(tuple_key.into()),
            authorization_model_id: self.authorization_model_id().to_string(),
            consistency: self.consistency().into(),
            contextual_tuples: contextual_tuples
                .into()
                .map(|tuple_keys| ContextualTupleKeys { tuple_keys }),
        };
        let response = self
            .client
            .clone()
            .expand(expand_request.clone())
            .await
            .map_err(|e| {
                tracing::error!(
                    "Expand request failed with status {e}. Request: {expand_request:?}"
                );
                Error::RequestFailed(e)
            })?;
        Ok(response.into_inner().tree)
    }

    /// Simplified version of [`Self::check`] without contextual tuples, context, or trace.
    ///
    /// # Errors
    /// Check the [`Self::check`] method for possible errors.
    pub async fn check_simple(&self, tuple_key: impl Into<CheckRequestTupleKey>) -> Result<bool> {
        self.check(tuple_key, None, None, false).await
    }

    /// List all objects of the given type that the user has a relation with.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] if the list-objects request fails
    pub async fn list_objects(
        &self,
        r#type: impl Into<String>,
        relation: impl Into<String>,
        user: impl Into<String>,
        contextual_tuples: impl Into<Option<Vec<TupleKey>>>,
        context: impl Into<Option<prost_wkt_types::Struct>>,
    ) -> Result<tonic::Response<ListObjectsResponse>> {
        let request = ListObjectsRequest {
            r#type: r#type.into(),
            relation: relation.into(),
            user: user.into(),
            authorization_model_id: self.authorization_model_id().to_string(),
            store_id: self.store_id().to_string(),
            consistency: self.consistency().into(),
            contextual_tuples: contextual_tuples
                .into()
                .map(|tuple_keys| ContextualTupleKeys { tuple_keys }),
            context: context.into(),
        };

        self.client
            .clone()
            .list_objects(request.clone())
            .await
            .map_err(|e| {
                tracing::error!(
                    "List-Objects request failed with status {e}. Request: {request:?}"
                );
                Error::RequestFailed(e)
            })
    }

    /// Delete all relations that other entities have to the given `object`, that
    /// is, all tuples with the "object" field set to the given `object`.
    ///
    /// This method uses streamed pagination internally, so that also large amounts of tuples can be deleted.
    /// Please not that this method does not delete tuples where the given object has a relation TO another entity.
    ///
    /// Iteration is stopped when no more tuples are returned from OpenFGA.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] if a read or delete request fails
    ///
    pub async fn delete_relations_to_object(&self, object: &str) -> Result<()> {
        loop {
            self.delete_relations_to_object_inner(object)
                .await
                .inspect_err(|e| {
                    tracing::error!("Failed to delete relations to object {object}: {e}");
                })?;

            if self.exists_relation_to(object).await? {
                tracing::debug!("Some tuples for object {object} are still present after first sweep. Performing another deletion.");
            } else {
                tracing::debug!("Successfully deleted all relations to object {object}");
                break Ok(());
            }
        }
    }

    /// Check if any direct relation to the given object exists.
    /// This does not check if the object is used as a user in relations to other objects.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] if the read request fails
    pub async fn exists_relation_to(&self, object: &str) -> Result<bool> {
        let tuples = self.read_relations_to_object(object, None, 1).await?;
        Ok(!tuples.tuples.is_empty())
    }

    async fn read_relations_to_object(
        &self,
        object: &str,
        continuation_token: impl Into<Option<String>>,
        page_size: i32,
    ) -> Result<ReadResponse> {
        self.read(
            page_size,
            TupleKeyWithoutCondition {
                user: String::new(),
                relation: String::new(),
                object: object.to_string(),
            },
            continuation_token,
        )
        .await
        .map(tonic::Response::into_inner)
    }

    /// # Errors
    /// * [`Error::RequestFailed`] if a read or delete request fails
    ///
    async fn delete_relations_to_object_inner(&self, object: &str) -> Result<()> {
        let read_stream = stream! {
            let mut continuation_token = None;
            // We need to keep track of seen keys, as OpenFGA might return
            // duplicates even of `HigherConsistency`.
            let mut seen= HashSet::new();
            while continuation_token != Some(String::new()) {
                let response = self.read_relations_to_object(object, continuation_token, self.max_tuples_per_write()).await?;
                let keys = response.tuples.into_iter().filter_map(|t| t.key).filter(|k| !seen.contains(&(k.user.clone(), k.relation.clone()))).collect::<Vec<_>>();
                tracing::debug!("Read {} keys for object {object} that are up for deletion. Continuation token: {}", keys.len(), response.continuation_token);
                continuation_token = Some(response.continuation_token);
                seen.extend(keys.iter().map(|k| (k.user.clone(), k.relation.clone())));
                yield Result::Ok(keys);
            }
        };
        pin_mut!(read_stream);
        let mut read_tuples: Option<Vec<TupleKey>> = None;

        let delete_tuples = |t: Option<Vec<TupleKey>>| async {
            match t {
                Some(tuples) => {
                    tracing::debug!(
                        "Deleting {} tuples for object {object} that we haven't seen before.",
                        tuples.len()
                    );
                    self.write(
                        None,
                        Some(
                            tuples
                                .into_iter()
                                .map(|t| TupleKeyWithoutCondition {
                                    user: t.user,
                                    relation: t.relation,
                                    object: t.object,
                                })
                                .collect(),
                        ),
                    )
                    .await
                }
                None => Ok(()),
            }
        };

        loop {
            let next_future = read_stream.next();
            let deletion_future = delete_tuples(read_tuples.clone());

            let (tuples, delete) = futures::join!(next_future, deletion_future);
            delete?;

            if let Some(tuples) = tuples.transpose()? {
                read_tuples = (!tuples.is_empty()).then_some(tuples);
            } else {
                break Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use needs_env_var::needs_env_var;

    #[needs_env_var(TEST_OPENFGA_CLIENT_GRPC_URL)]
    mod openfga {
        use tracing_test::traced_test;

        use super::super::*;
        use crate::{
            client::{AuthorizationModel, Store},
            migration::test::openfga::service_client_with_store,
        };

        async fn write_custom_roles_model(
            client: &OpenFgaServiceClient<tonic::transport::Channel>,
            store: &Store,
        ) -> String {
            let model: AuthorizationModel = serde_json::from_str(include_str!(
                "../tests/sample-store/custom-roles/schema.json"
            ))
            .unwrap();
            client
                .clone()
                .write_authorization_model(model.into_write_request(store.id.clone()))
                .await
                .unwrap()
                .into_inner()
                .authorization_model_id
        }

        async fn get_client_with_custom_roles_model() -> OpenFgaClient<tonic::transport::Channel> {
            let (service_client, store) = service_client_with_store().await;
            let auth_model_id = write_custom_roles_model(&service_client, &store).await;
            let client = OpenFgaClient::new(service_client, &store.id, auth_model_id.as_str());
            client
        }

        #[tokio::test]
        #[traced_test]
        async fn test_delete_relations_to_object() {
            let client = get_client_with_custom_roles_model().await;
            let object = "team:team1";

            assert!(!client.exists_relation_to(object).await.unwrap());

            client
                .write(
                    vec![TupleKey {
                        user: "user:user1".to_string(),
                        relation: "member".to_string(),
                        object: object.to_string(),
                        condition: None,
                    }],
                    None,
                )
                .await
                .unwrap();
            assert!(client.exists_relation_to(object).await.unwrap());
            client.delete_relations_to_object(object).await.unwrap();
            assert!(!client.exists_relation_to(object).await.unwrap());
        }

        #[tokio::test]
        #[traced_test]
        async fn test_delete_relations_to_object_usersets() {
            let client = get_client_with_custom_roles_model().await;
            let object: &str = "role:admin";

            assert!(!client.exists_relation_to(object).await.unwrap());

            client
                .write(
                    vec![TupleKey {
                        user: "team:team1#member".to_string(),
                        relation: "assignee".to_string(),
                        object: object.to_string(),
                        condition: None,
                    }],
                    None,
                )
                .await
                .unwrap();
            assert!(client.exists_relation_to(object).await.unwrap());
            client.delete_relations_to_object(object).await.unwrap();
            assert!(!client.exists_relation_to(object).await.unwrap());
        }

        #[tokio::test]
        #[traced_test]
        async fn test_delete_relations_to_object_empty() {
            let client = get_client_with_custom_roles_model().await;
            let object = "team:team1";

            assert!(!client.exists_relation_to(object).await.unwrap());
            client.delete_relations_to_object(object).await.unwrap();
            assert!(!client.exists_relation_to(object).await.unwrap());
        }

        #[tokio::test]
        #[traced_test]
        async fn test_delete_relations_to_object_many() {
            let client = get_client_with_custom_roles_model().await;
            let object = "org:org1";

            assert!(!client.exists_relation_to(object).await.unwrap());

            for i in 0..502 {
                client
                    .write(
                        vec![
                            TupleKey {
                                user: format!("user:user{i}"),
                                relation: "member".to_string(),
                                object: object.to_string(),
                                condition: None,
                            },
                            TupleKey {
                                user: format!("role:role{i}#assignee"),
                                relation: "role_assigner".to_string(),
                                object: object.to_string(),
                                condition: None,
                            },
                        ],
                        None,
                    )
                    .await
                    .unwrap();
            }

            // Also write a tuple for another org to make sure we don't delete those
            let object_2 = "org:org2";
            client
                .write(
                    vec![TupleKey {
                        user: "user:user1".to_string(),
                        relation: "owner".to_string(),
                        object: object_2.to_string(),
                        condition: None,
                    }],
                    None,
                )
                .await
                .unwrap();

            assert!(client.exists_relation_to(object).await.unwrap());
            assert!(client.exists_relation_to(object_2).await.unwrap());

            client.delete_relations_to_object(object).await.unwrap();

            assert!(!client.exists_relation_to(object).await.unwrap());
            assert!(client.exists_relation_to(object_2).await.unwrap());
            assert!(client
                .check_simple(TupleKeyWithoutCondition {
                    user: "user:user1".to_string(),
                    relation: "role_assigner".to_string(),
                    object: object_2.to_string(),
                })
                .await
                .unwrap());
        }
    }
}
