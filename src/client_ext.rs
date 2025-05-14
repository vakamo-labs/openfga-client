#![allow(unused_imports)]

use tonic::{
    codegen::{Body, Bytes, StdError},
    service::interceptor::InterceptorLayer,
    transport::{Channel, Endpoint},
};
#[cfg(feature = "auth-middle")]
use tower::{util::Either, ServiceBuilder};

use crate::{
    client::{OpenFgaClient, OpenFgaServiceClient},
    error::{Error, Result},
    generated::{
        ConsistencyPreference, CreateStoreRequest, ListStoresRequest, ReadRequest,
        ReadRequestTupleKey, Store, Tuple,
    },
};

#[cfg(feature = "auth-middle")]
/// Specialization of the [`OpenFgaServiceClient`] that includes optional
/// authentication with pre-shared keys (Bearer tokens) or client credentials.
/// For more fine-granular control, you can construct [`OpenFgaServiceClient`] directly
/// using interceptors for Authentication.
pub type BasicOpenFgaServiceClient = OpenFgaServiceClient<BasicAuthLayer>;

#[cfg(feature = "auth-middle")]
impl BasicOpenFgaServiceClient {
    /// Create a new client without authentication.
    ///
    /// # Errors
    /// * [`Error::InvalidEndpoint`] if the endpoint is not a valid URL.
    pub fn new_unauthenticated(endpoint: impl Into<url::Url>) -> Result<Self> {
        let either_or_option: EitherOrOption = None;
        let auth_layer = tower::util::option_layer(either_or_option);
        let endpoint = get_tonic_endpoint_logged(&endpoint.into())?;
        let c = ServiceBuilder::new()
            .layer(auth_layer)
            .service(endpoint.connect_lazy());
        Ok(BasicOpenFgaServiceClient::new(c))
    }

    /// Create a new client without authentication.
    ///
    /// # Errors
    /// * [`Error::InvalidEndpoint`] if the endpoint is not a valid URL.
    /// * [`Error::InvalidToken`] if the token is not valid ASCII.
    pub fn new_with_basic_auth(endpoint: impl Into<url::Url>, token: &str) -> Result<Self> {
        let either_or_option: EitherOrOption =
            Some(tower::util::Either::Right(tonic::service::interceptor(
                middle::BearerTokenAuthorizer::new(token).map_err(|e| {
                    tracing::error!("Could not construct OpenFGA client. Invalid token: {e}");
                    Error::InvalidToken {
                        reason: e.to_string(),
                    }
                })?,
            )));
        let auth_layer = tower::util::option_layer(either_or_option);
        let endpoint = get_tonic_endpoint_logged(&endpoint.into())?;
        let c = ServiceBuilder::new()
            .layer(auth_layer)
            .service(endpoint.connect_lazy());
        Ok(BasicOpenFgaServiceClient::new(c))
    }

    /// Create a new client using client credentials.
    ///
    /// # Errors
    /// * [`Error::InvalidEndpoint`] if the endpoint is not a valid URL.
    /// * [`Error::CredentialRefreshError`] if the client credentials could not be exchanged for a token.
    pub async fn new_with_client_credentials(
        endpoint: impl Into<url::Url>,
        client_id: &str,
        client_secret: &str,
        token_endpoint: impl Into<url::Url>,
        scopes: &[&str],
    ) -> Result<Self> {
        let either_or_option: EitherOrOption =
            Some(tower::util::Either::Left(tonic::service::interceptor(
                {
                let builder = middle::BasicClientCredentialAuthorizer::basic_builder(
                    client_id,
                    client_secret,
                    token_endpoint.into(),
                );
                if scopes.is_empty() {
                    builder
                } else {
                    builder.add_scopes(scopes)
                }
            }
                .build()
                .await.map_err(|e| {
                    tracing::error!("Could not construct OpenFGA client. Failed to fetch or refresh Client Credentials: {e}");
                    Error::CredentialRefreshError(e)
                })?,
            )));
        let auth_layer = tower::util::option_layer(either_or_option);
        let endpoint = get_tonic_endpoint_logged(&endpoint.into())?;
        let c = ServiceBuilder::new()
            .layer(auth_layer)
            .service(endpoint.connect_lazy());
        Ok(BasicOpenFgaServiceClient::new(c))
    }
}

impl<T> OpenFgaServiceClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    /// Transform this service client into a higher-level [`OpenFgaClient`].
    pub fn into_client(self, store_id: &str, authorization_model_id: &str) -> OpenFgaClient<T> {
        OpenFgaClient::new(self, store_id, authorization_model_id)
    }

    /// Fetch a store by name.
    /// If no store is found, returns `Ok(None)`.
    ///
    /// # Errors
    /// * [`Error::AmbiguousStoreName`] if multiple stores with the same name are found.
    /// * [`Error::RequestFailed`] if the request to OpenFGA fails.
    pub async fn get_store_by_name(&mut self, store_name: &str) -> Result<Option<Store>> {
        let stores = self
            .list_stores(ListStoresRequest {
                page_size: Some(2),
                continuation_token: String::new(),
                name: store_name.to_string(),
            })
            .await
            .map_err(|e| {
                tracing::error!("Failed to list stores in OpenFGA: {e}");
                Error::RequestFailed(e)
            })?
            .into_inner();
        let num_stores = stores.stores.len();

        match stores.stores.first() {
            Some(store) => {
                if num_stores > 1 {
                    tracing::error!("Multiple stores with the name `{}` found", store_name);
                    Err(Error::AmbiguousStoreName(store_name.to_string()))
                } else {
                    Ok(Some(store.clone()))
                }
            }
            None => Ok(None),
        }
    }

    /// Get a store by name or create it if it doesn't exist.
    /// Returns information about the store, including its ID.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] If a request to OpenFGA fails.
    /// * [`Error::AmbiguousStoreName`] If multiple stores with the same name are found.
    pub async fn get_or_create_store(&mut self, store_name: &str) -> Result<Store> {
        let store = self.get_store_by_name(store_name).await?;
        match store {
            None => {
                tracing::debug!("OpenFGA Store {} not found. Creating it.", store_name);
                let store = self
                    .create_store(CreateStoreRequest {
                        name: store_name.to_owned(),
                    })
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to create store in OpenFGA: {e}");
                        Error::RequestFailed(e)
                    })?
                    .into_inner();
                Ok(Store {
                    id: store.id,
                    name: store.name,
                    created_at: store.created_at,
                    updated_at: store.updated_at,
                    deleted_at: None,
                })
            }
            Some(store) => Ok(store),
        }
    }

    /// Wrapper around [`Self::read`] that reads all pages of the result, handling pagination.
    ///
    /// # Errors
    /// * [`Error::RequestFailed`] If a request to OpenFGA fails.
    /// * [`Error::TooManyPages`] If the number of pages read exceeds `max_pages`.
    pub async fn read_all_pages(
        &mut self,
        store_id: &str,
        tuple: impl Into<ReadRequestTupleKey>,
        consistency: impl Into<ConsistencyPreference>,
        page_size: i32,
        max_pages: u32,
    ) -> Result<Vec<Tuple>> {
        let mut continuation_token = String::new();
        let mut tuples = Vec::new();
        let mut count = 0;
        let tuple = tuple.into();
        let consistency = consistency.into();

        loop {
            let read_request = ReadRequest {
                store_id: store_id.to_owned(),
                tuple_key: Some(tuple.clone()),
                page_size: Some(page_size),
                continuation_token: continuation_token.clone(),
                consistency: consistency.into(),
            };
            let response = self
                .read(read_request.clone())
                .await
                .map_err(|e| {
                    tracing::error!(
                        "Failed to read from OpenFGA: {e}. Request: {:?}",
                        read_request
                    );
                    Error::RequestFailed(e)
                })?
                .into_inner();
            tuples.extend(response.tuples);
            continuation_token.clone_from(&response.continuation_token);
            if continuation_token.is_empty() || count > max_pages {
                if count > max_pages {
                    return Err(Error::TooManyPages { max_pages, tuple });
                }
                break;
            }
            count += 1;
        }

        Ok(tuples)
    }
}

#[cfg(feature = "auth-middle")]
pub type BasicAuthLayer = tower::util::Either<
    tower::util::Either<
        tonic::service::interceptor::InterceptedService<
            Channel,
            middle::BasicClientCredentialAuthorizer,
        >,
        tonic::service::interceptor::InterceptedService<Channel, middle::BearerTokenAuthorizer>,
    >,
    Channel,
>;

#[cfg(feature = "auth-middle")]
type EitherOrOption = Option<
    Either<
        InterceptorLayer<middle::BasicClientCredentialAuthorizer>,
        InterceptorLayer<middle::BearerTokenAuthorizer>,
    >,
>;

#[cfg(feature = "auth-middle")]
fn get_tonic_endpoint_logged(endpoint: &url::Url) -> Result<Endpoint> {
    Endpoint::new(endpoint.to_string()).map_err(|e| {
        tracing::error!("Could not construct OpenFGA client. Invalid endpoint `{endpoint}`: {e}");
        Error::InvalidEndpoint(endpoint.to_string())
    })
}

#[cfg(test)]
pub(crate) mod test {
    use needs_env_var::needs_env_var;

    // #[needs_env_var(TEST_OPENFGA_CLIENT_GRPC_URL)]
    #[cfg(feature = "auth-middle")]
    mod openfga {
        use std::collections::{HashMap, HashSet};

        use super::super::*;
        use crate::{
            client::{
                TupleKey, WriteAuthorizationModelRequest, WriteAuthorizationModelResponse,
                WriteRequest, WriteRequestWrites,
            },
            generated::AuthorizationModel,
        };

        fn get_basic_client() -> BasicOpenFgaServiceClient {
            let endpoint = std::env::var("TEST_OPENFGA_CLIENT_GRPC_URL").unwrap();
            BasicOpenFgaServiceClient::new_unauthenticated(url::Url::parse(&endpoint).unwrap())
                .expect("Client can be created")
        }

        async fn new_store() -> (BasicOpenFgaServiceClient, Store) {
            let mut client = get_basic_client();
            let store_name = format!("store-{}", uuid::Uuid::now_v7());
            let store = client
                .get_or_create_store(&store_name)
                .await
                .expect("Store can be created");
            (client, store)
        }

        async fn create_entitlements_model(
            client: &mut BasicOpenFgaServiceClient,
            store: &Store,
        ) -> WriteAuthorizationModelResponse {
            let schema = include_str!("../tests/sample-store/entitlements/schema.json");
            let model: AuthorizationModel =
                serde_json::from_str(schema).expect("Schema can be deserialized");
            let auth_model = client
                .write_authorization_model(WriteAuthorizationModelRequest {
                    store_id: store.id.clone(),
                    type_definitions: model.type_definitions,
                    schema_version: model.schema_version,
                    conditions: model.conditions,
                })
                .await
                .expect("Auth model can be written");

            auth_model.into_inner()
        }

        #[tokio::test]
        async fn test_get_store_by_name_many() {
            let mut client = get_basic_client();

            let mut stores = HashMap::new();
            for _i in 0..201 {
                let store_name = format!("store-{}", uuid::Uuid::now_v7());
                let r = client
                    .get_or_create_store(&store_name)
                    .await
                    .expect("Store can be created");
                assert_eq!(store_name, r.name);
                stores.insert(store_name, r.id);
            }

            for (store_name, store_id) in stores {
                let store = client
                    .get_store_by_name(&store_name)
                    .await
                    .expect("Store can be fetched")
                    .expect("Store exists");
                assert_eq!(store_id, store.id);
            }
        }

        #[tokio::test]
        async fn test_get_store_by_name_non_existant() {
            let mut client = get_basic_client();
            let store = client
                .get_store_by_name("non-existent-store")
                .await
                .unwrap();
            assert!(store.is_none());
        }

        #[tokio::test]
        async fn test_get_or_create_store() {
            let mut client = get_basic_client();
            let store_name = format!("store-{}", uuid::Uuid::now_v7());
            let store = client.get_or_create_store(&store_name).await.unwrap();
            let store2 = client.get_or_create_store(&store_name).await.unwrap();
            assert_eq!(store.id, store2.id);
        }

        #[tokio::test]
        async fn test_read_all_pages_many() {
            let (mut client, store) = new_store().await;
            let auth_model = create_entitlements_model(&mut client, &store).await;
            let object = "organization:org-1";

            let users = (0..501)
                .map(|i| format!("user:u-{i}"))
                .collect::<Vec<String>>();

            for user in &users {
                client
                    .write(WriteRequest {
                        authorization_model_id: auth_model.authorization_model_id.clone(),
                        store_id: store.id.clone(),
                        writes: Some(WriteRequestWrites {
                            tuple_keys: vec![TupleKey {
                                user: user.to_string(),
                                relation: "member".to_string(),
                                object: object.to_string(),
                                condition: None,
                            }],
                        }),
                        deletes: None,
                    })
                    .await
                    .expect("Write can be done");
            }

            let tuples = client
                .read_all_pages(
                    &store.id,
                    ReadRequestTupleKey {
                        user: String::new(),
                        relation: "member".to_string(),
                        object: object.to_string(),
                    },
                    ConsistencyPreference::HigherConsistency,
                    100,
                    6,
                )
                .await
                .expect("Read can be done");

            assert_eq!(tuples.len(), 501);
            assert_eq!(
                tuples
                    .iter()
                    .map(|t| t.key.clone().unwrap().user)
                    .collect::<HashSet<String>>(),
                HashSet::from_iter(users)
            );
        }

        #[tokio::test]
        async fn test_real_all_pages_empty() {
            let (mut client, store) = new_store().await;
            let tuples = client
                .read_all_pages(
                    &store.id,
                    ReadRequestTupleKey {
                        user: String::new(),
                        relation: "member".to_string(),
                        object: "organization:org-1".to_string(),
                    },
                    ConsistencyPreference::HigherConsistency,
                    100,
                    5,
                )
                .await
                .expect("Read can be done");

            assert!(tuples.is_empty());
        }
    }
}
