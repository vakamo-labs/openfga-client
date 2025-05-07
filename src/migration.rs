use std::{
    collections::{HashMap, HashSet},
    future::Future,
    hash::Hash,
    pin::Pin,
    str::FromStr,
    sync::Arc,
};

use tonic::codegen::{Body, Bytes, StdError};

use crate::{
    client::{
        AuthorizationModel, ConsistencyPreference, OpenFgaServiceClient, ReadRequestTupleKey,
        Store, Tuple, TupleKey, WriteAuthorizationModelResponse, WriteRequest, WriteRequestWrites,
    },
    error::{Error, Result},
};

const DEFAULT_PAGE_SIZE: i32 = 100;
const MAX_PAGES: u32 = 1000;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct AuthorizationModelVersion {
    major: u32,
    minor: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct VersionedAuthorizationModel {
    model: AuthorizationModel,
    version: AuthorizationModelVersion,
}

/// Manages [`AuthorizationModel`]s in OpenFGA.
///
/// Authorization models in OpenFGA don't receive a unique name. Instead,
/// they receive a random id on creation. If we don't store this ID, we can't
/// find the model again and use its ID.
///
/// This `ModelManager` stores the mapping of [`AuthorizationModelVersion`]
/// to the ID of the model in OpenFGA directly inside OpenFGA.
/// This way can query OpenFGA to determine if a model with a certain version
/// has already exists.
///
/// When running [`TupleModelManager::migrate()`], the manager only applies models and their migrations
/// if they don't already exist in OpenFGA.
///
/// To store the mapping of model versions to OpenFGA IDs, the following needs to part of your Authorization Model:
/// ```text
/// type auth_model_id
/// type model_version
///   relations
///     define openfga_id: [auth_model_id]
///     define exists: [auth_model_id:*]
/// ```
///
#[derive(Debug, Clone)]
pub struct TupleModelManager<T>
where
    T: tonic::client::GrpcService<tonic::body::Body>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    client: OpenFgaServiceClient<T>,
    store_name: String,
    model_prefix: String,
    migrations: HashMap<AuthorizationModelVersion, Migration<T>>,
}

#[derive(Clone)]
struct Migration<T> {
    model: VersionedAuthorizationModel,
    pre_migration_fn: Option<BoxedMigrationFn<T>>,
    post_migration_fn: Option<BoxedMigrationFn<T>>,
}

// Define a type alias for a boxed future with a specific lifetime
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Possible function pointer that implements the migration function signature.
pub type MigrationFn<T> =
    fn(OpenFgaServiceClient<T>) -> BoxFuture<'static, std::result::Result<(), StdError>>;

/// Type alias for the migration function signature.
type DynMigrationFn<T> =
    dyn Fn(OpenFgaServiceClient<T>) -> BoxFuture<'static, std::result::Result<(), StdError>>;

/// Boxed migration function
type BoxedMigrationFn<T> = Arc<DynMigrationFn<T>>;

// Function to box the async functions that take an i32 parameter
fn box_migration_fn<T, F, Fut>(f: F) -> BoxedMigrationFn<T>
where
    F: Fn(OpenFgaServiceClient<T>) -> Fut + Send + 'static,
    Fut: Future<Output = std::result::Result<(), StdError>> + Send + 'static,
{
    Arc::new(move |v| Box::pin(f(v)))
}

impl<T> TupleModelManager<T>
where
    T: tonic::client::GrpcService<tonic::body::Body>,
    T: Clone,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
{
    const AUTH_MODEL_ID_TYPE: &'static str = "auth_model_id";
    const MODEL_VERSION_EXISTS_RELATION: &'static str = "exists";
    const MODEL_VERSION_TYPE: &'static str = "model_version";
    const MODEL_VERSION_OPENFGA_ID_RELATION: &'static str = "openfga_id";

    /// Create a new `TupleModelManager` with the given client and model name.
    /// The model prefix must not change after the first model has been added or
    /// the model manager will not be able to find the model again.
    /// Use different model prefixes if models for different purposes are stored in the
    /// same OpenFGA store.
    pub fn new(client: OpenFgaServiceClient<T>, store_name: &str, model_prefix: &str) -> Self {
        TupleModelManager {
            client,
            model_prefix: model_prefix.to_string(),
            store_name: store_name.to_string(),
            migrations: HashMap::new(),
        }
    }

    /// Add a new model to the manager.
    /// If a model with the same version has already been added, the new model will replace the old one.
    ///
    /// Ensure that migration functions are written in an idempotent way.
    /// If a migration fails, it might be retried.
    #[must_use]
    pub fn add_model<FutPre, FutPost>(
        mut self,
        model: AuthorizationModel,
        version: AuthorizationModelVersion,
        pre_migration_fn: Option<impl Fn(OpenFgaServiceClient<T>) -> FutPre + Send + 'static>,
        post_migration_fn: Option<impl Fn(OpenFgaServiceClient<T>) -> FutPost + Send + 'static>,
    ) -> Self
    where
        FutPre: Future<Output = std::result::Result<(), StdError>> + Send + 'static,
        FutPost: Future<Output = std::result::Result<(), StdError>> + Send + 'static,
    {
        let migration = Migration {
            model: VersionedAuthorizationModel::new(model, version),
            pre_migration_fn: pre_migration_fn.map(box_migration_fn),
            post_migration_fn: post_migration_fn.map(box_migration_fn),
        };
        self.migrations.insert(migration.model.version(), migration);
        self
    }

    /// Run migrations.
    ///
    /// This will:
    /// 1. Get all existing models in the OpenFGA store.
    /// 2. Determine which migrations need to be performed: All migrations with a version higher than the highest existing model.
    /// 3. In order of the version of the model, perform the migrations:
    ///    1. Run the pre-migration hook if it exists.
    ///    2. Write the model to OpenFGA.
    ///    3. Run the post-migration hook if it exists.
    /// 4. Mark the model as applied in OpenFGA.
    ///
    /// # Errors
    /// * If OpenFGA cannot be reached or a request fails.
    /// * If any of the migration hooks fail.
    pub async fn migrate(&mut self) -> Result<()> {
        let span = tracing::span!(
            tracing::Level::INFO,
            "Running OpenFGA Migrations",
            store_name = self.store_name,
            model_prefix = self.model_prefix
        );
        let _enter = span.enter();

        if self.migrations.is_empty() {
            tracing::info!("No Migrations have been added. Nothing to do.");
            return Ok(());
        }

        let store = self.client.get_or_create_store(&self.store_name).await?;
        let existing_models = self.get_existing_versions().await?;
        let max_existing_model = existing_models.iter().max();

        if let Some(max_existing_model) = max_existing_model {
            tracing::info!(
                "Currently the highest existing Model Version is: {}",
                max_existing_model
            );
        } else {
            tracing::info!("No model found in OpenFGA store");
        }

        let ordered_migrations = self.migrations_to_perform(max_existing_model.copied());

        let mut client = self.client.clone();
        for migration in ordered_migrations {
            tracing::info!("Migrating to model version: {}", migration.model.version());

            // Pre-hook
            if let Some(pre_migration_fn) = migration.pre_migration_fn.as_ref() {
                pre_migration_fn(client.clone()).await.map_err(|e| {
                    tracing::error!("Error in OpenFGA pre-migration hook: {:?}", e);
                    Error::MigrationHookFailed {
                        version: migration.model.version().to_string(),
                        error: Arc::new(e),
                    }
                })?;
            }

            // Write Model
            let request = migration
                .model
                .model()
                .clone()
                .into_write_request(store.id.clone());
            let written_model = client
                .write_authorization_model(request)
                .await
                .map_err(|e| {
                    tracing::error!("Error writing model: {:?}", e);
                    Error::RequestFailed(e)
                })?;
            tracing::debug!("Model written: {:?}", written_model);

            // Post-hook
            if let Some(post_migration_fn) = migration.post_migration_fn.as_ref() {
                post_migration_fn(client.clone()).await.map_err(|e| {
                    tracing::error!("Error in OpenFGA post-migration hook: {:?}", e);
                    Error::MigrationHookFailed {
                        version: migration.model.version().to_string(),
                        error: Arc::new(e),
                    }
                })?;
            }

            // Mark as applied
            Self::mark_as_applied(
                &mut client,
                &self.model_prefix,
                &store,
                migration.model.version(),
                written_model.into_inner(),
            )
            .await?;
        }

        Ok(())
    }

    /// Get the OpenFGA Authorization model ID for the specified model version.
    /// Ensure that migrations have been run before calling this method.
    ///
    /// # Errors
    /// * If the store with the given name does not exist.
    /// * If a call to OpenFGA fails.
    pub async fn get_authorization_model_id(
        &mut self,
        version: AuthorizationModelVersion,
    ) -> Result<Option<String>> {
        let store = self
            .client
            .get_store_by_name(&self.store_name)
            .await?
            .ok_or_else(|| {
                tracing::error!("Store with name {} not found", self.store_name);
                Error::StoreNotFound(self.store_name.clone())
            })?;

        let applied_models = self
            .client
            .read_all_pages(
                &store.id,
                ReadRequestTupleKey {
                    user: String::new(),
                    relation: Self::MODEL_VERSION_OPENFGA_ID_RELATION.to_string(),
                    object: Self::format_model_version_key(&self.model_prefix, version),
                },
                ConsistencyPreference::HigherConsistency,
                DEFAULT_PAGE_SIZE,
                MAX_PAGES,
            )
            .await?;

        let applied_models = applied_models
            .into_iter()
            .filter_map(|t| t.key)
            .filter_map(|t| {
                t.user
                    .strip_prefix(&format!("{}:", Self::AUTH_MODEL_ID_TYPE))
                    .map(ToString::to_string)
            })
            .collect::<Vec<_>>();

        if applied_models.len() > 1 {
            tracing::error!(
                "Multiple authorization models with model prefix {} for version {} found.",
                self.model_prefix,
                version
            );
            return Err(Error::AmbiguousModelVersion {
                model_prefix: self.model_prefix.clone(),
                version: version.to_string(),
            });
        }

        let model_id = applied_models.into_iter().next().map(|openfga_id| {
            tracing::info!(
                "Authorization model for version {version} found in OpenFGA store {}. Model ID: {openfga_id}",
                self.store_name,
            );
            openfga_id
        });

        Ok(model_id)
    }

    /// Mark a model version as applied in OpenFGA
    async fn mark_as_applied(
        client: &mut OpenFgaServiceClient<T>,
        model_prefix: &str,
        store: &Store,
        version: AuthorizationModelVersion,
        write_response: WriteAuthorizationModelResponse,
    ) -> Result<()> {
        let authorization_model_id = write_response.authorization_model_id;
        let object = Self::format_model_version_key(model_prefix, version);

        let write_request = WriteRequest {
            store_id: store.id.clone(),
            writes: Some(WriteRequestWrites {
                tuple_keys: vec![
                    TupleKey {
                        user: format!("{}:{authorization_model_id}", Self::AUTH_MODEL_ID_TYPE),
                        relation: Self::MODEL_VERSION_OPENFGA_ID_RELATION.to_string(),
                        object: object.clone(),
                        condition: None,
                    },
                    TupleKey {
                        user: format!("{}:*", Self::AUTH_MODEL_ID_TYPE),
                        relation: Self::MODEL_VERSION_EXISTS_RELATION.to_string(),
                        object,
                        condition: None,
                    },
                ],
            }),
            deletes: None,
            authorization_model_id: authorization_model_id.to_string(),
        };
        client.write(write_request.clone()).await.map_err(|e| {
            tracing::error!("Error marking model as applied: {:?}", e);
            Error::RequestFailed(e)
        })?;
        Ok(())
    }

    /// Get all migrations that have been added to the manager
    /// as a `Vec` sorted by the version of the model.
    fn ordered_migrations(&self) -> Vec<&Migration<T>> {
        let mut migrations = self.migrations.values().collect::<Vec<_>>();
        migrations.sort_unstable_by_key(|m| m.model.version());
        migrations
    }

    /// Get all migrations that need to be performed, given the maximum existing model version.
    fn migrations_to_perform(
        &self,
        max_existing_model: Option<AuthorizationModelVersion>,
    ) -> Vec<&Migration<T>> {
        let ordered_migrations = self.ordered_migrations();
        let migrations_to_perform = ordered_migrations
            .into_iter()
            .filter(|m| {
                max_existing_model.map_or(true, |max_existing| m.model.version() > max_existing)
            })
            .collect::<Vec<_>>();

        tracing::info!(
            "{} migrations needed in OpenFGA store {} for model-prefix {}",
            migrations_to_perform.len(),
            self.store_name,
            self.model_prefix
        );
        migrations_to_perform
    }

    /// Get versions of all existing models in OpenFGA.
    /// Returns an empty vector if the store does not exist.
    ///
    /// # Errors
    /// * If the call to determine existing stores fails.
    /// * If a tuple read call fails.
    pub async fn get_existing_versions(&mut self) -> Result<Vec<AuthorizationModelVersion>> {
        let Some(store) = self.client.get_store_by_name(&self.store_name).await? else {
            return Ok(vec![]);
        };

        let tuples = self
            .client
            .read_all_pages(
                &store.id,
                ReadRequestTupleKey {
                    user: format!("{}:*", Self::AUTH_MODEL_ID_TYPE).to_string(),
                    relation: Self::MODEL_VERSION_EXISTS_RELATION.to_string(),
                    object: format!("{}:", Self::MODEL_VERSION_TYPE).to_string(),
                },
                crate::client::ConsistencyPreference::HigherConsistency,
                DEFAULT_PAGE_SIZE,
                MAX_PAGES,
            )
            .await?;
        let existing_models = Self::parse_existing_models(tuples, &self.model_prefix);
        Ok(existing_models.into_iter().collect())
    }

    fn parse_existing_models(
        exist_tuples: Vec<Tuple>,
        model_prefix: &str,
    ) -> HashSet<AuthorizationModelVersion> {
        exist_tuples
            .into_iter()
            .filter_map(|t| t.key)
            .filter_map(|t| Self::parse_model_version_from_key(&t.object, model_prefix))
            .collect()
    }

    fn parse_model_version_from_key(
        model: &str,
        model_prefix: &str,
    ) -> Option<AuthorizationModelVersion> {
        model
            // Ignore models with wrong prefix
            .strip_prefix(&format!("{}:", Self::MODEL_VERSION_TYPE))
            .and_then(|model| {
                model
                    .strip_prefix(&format!("{model_prefix}-"))
                    .and_then(|version| AuthorizationModelVersion::from_str(version).ok())
            })
    }

    fn format_model_version_key(model_prefix: &str, version: AuthorizationModelVersion) -> String {
        format!("{}:{}-{}", Self::MODEL_VERSION_TYPE, model_prefix, version)
    }
}

impl VersionedAuthorizationModel {
    pub(crate) fn new(model: AuthorizationModel, version: AuthorizationModelVersion) -> Self {
        VersionedAuthorizationModel { model, version }
    }

    pub(crate) fn version(&self) -> AuthorizationModelVersion {
        self.version
    }

    pub(crate) fn model(&self) -> &AuthorizationModel {
        &self.model
    }
}

impl AuthorizationModelVersion {
    #[must_use]
    pub fn new(major: u32, minor: u32) -> Self {
        AuthorizationModelVersion { major, minor }
    }

    #[must_use]
    pub fn major(&self) -> u32 {
        self.major
    }

    #[must_use]
    pub fn minor(&self) -> u32 {
        self.minor
    }
}

// Sort by major version first, then by subversion.
impl PartialOrd for AuthorizationModelVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AuthorizationModelVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.major, self.minor).cmp(&(other.major, other.minor))
    }
}

impl std::fmt::Display for AuthorizationModelVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl FromStr for AuthorizationModelVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split('.').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(Error::InvalidModelVersion(s.to_string()));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| Error::InvalidModelVersion(s.to_string()))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| Error::InvalidModelVersion(s.to_string()))?;

        Ok(AuthorizationModelVersion::new(major, minor))
    }
}

impl<T> std::fmt::Debug for Migration<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Migration")
            .field("model", &self.model)
            .field("pre_migration_fn", &"...")
            .field("post_migration_fn", &"...")
            .finish()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use std::sync::Mutex;

    use needs_env_var::needs_env_var;
    use pretty_assertions::assert_eq;

    use super::*;

    type ChannelTupleManager = TupleModelManager<tonic::transport::Channel>;

    #[test]
    fn test_ordering() {
        let versioned_1_0 = AuthorizationModelVersion::new(1, 0);
        let versioned_1_1 = AuthorizationModelVersion::new(1, 1);
        let versioned_2_0 = AuthorizationModelVersion::new(2, 0);
        let versioned_2_1 = AuthorizationModelVersion::new(2, 1);
        let versioned_2_2 = AuthorizationModelVersion::new(2, 2);

        assert!(versioned_1_0 < versioned_1_1);
        assert!(versioned_1_1 < versioned_2_0);
        assert!(versioned_2_0 < versioned_2_1);
        assert!(versioned_2_1 < versioned_2_2);
    }

    #[test]
    fn test_auth_model_version_str() {
        let version = AuthorizationModelVersion::new(1, 0);
        assert_eq!(version.to_string(), "1.0");
        assert_eq!("1.0".parse::<AuthorizationModelVersion>().unwrap(), version);

        let version = AuthorizationModelVersion::new(10, 2);
        assert_eq!(version.to_string(), "10.2");
        assert_eq!(
            "10.2".parse::<AuthorizationModelVersion>().unwrap(),
            version
        );
    }

    #[test]
    fn test_parse_model_version_from_key() {
        let model_prefix = "test";
        let model_version = AuthorizationModelVersion::new(1, 0);
        let key = format!("model_version:{model_prefix}-{model_version}");
        assert_eq!(
            ChannelTupleManager::parse_model_version_from_key(&key, model_prefix),
            Some(model_version)
        );

        // Prefix missing
        assert!(ChannelTupleManager::parse_model_version_from_key(
            "model_version:1.0",
            model_prefix
        )
        .is_none());

        // Wrong prefix
        assert!(ChannelTupleManager::parse_model_version_from_key(
            "model_version:foo-1.0",
            model_prefix
        )
        .is_none());

        // Higher version
        assert_eq!(
            ChannelTupleManager::parse_model_version_from_key(
                "model_version:other-model-10.200",
                "other-model"
            ),
            Some(AuthorizationModelVersion::new(10, 200))
        );
    }

    #[test]
    fn test_format_model_version_key() {
        let model_prefix = "test";
        let model_version = AuthorizationModelVersion::new(1, 0);
        let key = ChannelTupleManager::format_model_version_key(model_prefix, model_version);
        assert_eq!(key, "model_version:test-1.0");
        let parsed = ChannelTupleManager::parse_model_version_from_key(&key, model_prefix).unwrap();
        assert_eq!(parsed, model_version);
    }

    #[needs_env_var(TEST_OPENFGA_CLIENT_GRPC_URL)]
    pub(crate) mod openfga {
        use std::{str::FromStr, sync::Mutex};

        use pretty_assertions::assert_eq;

        use super::*;
        use crate::client::{OpenFgaServiceClient, ReadAuthorizationModelRequest};

        pub(crate) async fn get_service_client() -> OpenFgaServiceClient<tonic::transport::Channel>
        {
            let endpoint = std::env::var("TEST_OPENFGA_CLIENT_GRPC_URL").unwrap();
            let endpoint = tonic::transport::Endpoint::from_str(&endpoint).unwrap();
            OpenFgaServiceClient::connect(endpoint)
                .await
                .expect("Client can be created")
        }

        pub(crate) async fn service_client_with_store(
        ) -> (OpenFgaServiceClient<tonic::transport::Channel>, Store) {
            let mut client = get_service_client().await;
            let store_name = format!("test-{}", uuid::Uuid::now_v7());
            let store = client.get_or_create_store(&store_name).await.unwrap();
            (client, store)
        }

        #[tokio::test]
        async fn test_get_existing_versions_nonexistent_store() {
            let client = get_service_client().await;
            let mut manager = TupleModelManager::new(client, "nonexistent", "test");

            let versions = manager.get_existing_versions().await.unwrap();
            assert!(versions.is_empty());
        }

        #[tokio::test]
        async fn test_get_existing_versions_nonexistent_auth_model() {
            let mut client = get_service_client().await;
            let store_name = format!("test-{}", uuid::Uuid::now_v7());
            let _store = client.get_or_create_store(&store_name).await.unwrap();
            let mut manager = TupleModelManager::new(client, &store_name, "test");
            let versions = manager.get_existing_versions().await.unwrap();
            assert!(versions.is_empty());
        }

        #[tokio::test]
        async fn test_get_authorization_model_id() {
            let (mut client, store) = service_client_with_store().await;
            let model_prefix = "test";
            let version = AuthorizationModelVersion::new(1, 0);

            let mut manager = TupleModelManager::new(client.clone(), &store.name, model_prefix);

            // Non-existent model
            assert_eq!(
                manager.get_authorization_model_id(version).await.unwrap(),
                None
            );

            // Apply auth model
            let model: AuthorizationModel =
                serde_json::from_str(include_str!("../tests/model-manager/v1.0/schema.json"))
                    .unwrap();
            client
                .write_authorization_model(model.into_write_request(store.id.clone()))
                .await
                .unwrap();

            // Write model tuples
            client
                .write(WriteRequest {
                    store_id: store.id.clone(),
                    writes: Some(WriteRequestWrites {
                        tuple_keys: vec![
                            TupleKey {
                                user: "auth_model_id:111111".to_string(),
                                relation: "openfga_id".to_string(),
                                object: "model_version:test-1.0".to_string(),
                                condition: None,
                            },
                            TupleKey {
                                user: "auth_model_id:*".to_string(),
                                relation: "exists".to_string(),
                                object: "model_version:test-1.0".to_string(),
                                condition: None,
                            },
                            // Tuple with different model prefix should be ignored
                            TupleKey {
                                user: "auth_model_id:*".to_string(),
                                relation: "exists".to_string(),
                                object: "model_version:test2-1.0".to_string(),
                                condition: None,
                            },
                        ],
                    }),
                    deletes: None,
                    authorization_model_id: String::new(),
                })
                .await
                .unwrap();

            assert_eq!(
                manager.get_authorization_model_id(version).await.unwrap(),
                Some("111111".to_string())
            );
        }

        #[tokio::test]
        async fn test_model_manager() {
            let store_name = format!("test-{}", uuid::Uuid::now_v7());
            let mut client = get_service_client().await;

            let model_1_0: AuthorizationModel =
                serde_json::from_str(include_str!("../tests/model-manager/v1.0/schema.json"))
                    .unwrap();

            let version_1_0 = AuthorizationModelVersion::new(1, 0);
            let execution_counter_1 = Arc::new(Mutex::new(0));

            let execution_counter_clone = execution_counter_1.clone();
            let mut manager = TupleModelManager::new(client.clone(), &store_name, "test-model")
                .add_model(
                    model_1_0.clone(),
                    version_1_0,
                    Some(move |client| {
                        let counter = execution_counter_clone.clone();
                        async move { v1_pre_migration_fn(client, counter).await }
                    }),
                    None::<MigrationFn<_>>,
                );
            manager.migrate().await.unwrap();
            // Check hook was called once
            assert_eq!(*execution_counter_1.lock().unwrap(), 1);
            manager.migrate().await.unwrap();
            // Check hook was not called again
            assert_eq!(*execution_counter_1.lock().unwrap(), 1);

            // Check written model
            let auth_model_id = manager
                .get_authorization_model_id(version_1_0)
                .await
                .unwrap()
                .unwrap();
            let mut auth_model =
                get_auth_model_by_id(&mut client, &store_name, &auth_model_id).await;
            auth_model.id = model_1_0.id.clone();
            assert_eq!(
                serde_json::to_value(&model_1_0).unwrap(),
                serde_json::to_value(auth_model).unwrap()
            );

            // Add a second model
            let model_1_1: AuthorizationModel =
                serde_json::from_str(include_str!("../tests/model-manager/v1.1/schema.json"))
                    .unwrap();
            let version_1_1 = AuthorizationModelVersion::new(1, 1);
            let execution_counter_2 = Arc::new(Mutex::new(0));
            let execution_counter_clone = execution_counter_2.clone();
            let mut manager = manager.add_model(
                model_1_1.clone(),
                version_1_1,
                None::<MigrationFn<_>>,
                Some(move |client| {
                    let counter = execution_counter_clone.clone();
                    async move { v2_post_migration_fn(client, counter).await }
                }),
            );
            manager.migrate().await.unwrap();
            manager.migrate().await.unwrap();
            manager.migrate().await.unwrap();

            // First migration still only called once
            assert_eq!(*execution_counter_1.lock().unwrap(), 1);
            // Second migration called once
            assert_eq!(*execution_counter_2.lock().unwrap(), 1);

            // Check written model
            let auth_model_id = manager
                .get_authorization_model_id(version_1_1)
                .await
                .unwrap()
                .unwrap();
            let mut auth_model =
                get_auth_model_by_id(&mut client, &store_name, &auth_model_id).await;
            auth_model.id = model_1_1.id.clone();
            assert_eq!(
                serde_json::to_value(&model_1_1).unwrap(),
                serde_json::to_value(auth_model).unwrap()
            );
        }

        async fn get_auth_model_by_id(
            client: &mut OpenFgaServiceClient<tonic::transport::Channel>,
            store_name: &str,
            auth_model_id: &str,
        ) -> AuthorizationModel {
            client
                .read_authorization_model(ReadAuthorizationModelRequest {
                    store_id: client
                        .clone()
                        .get_store_by_name(store_name)
                        .await
                        .unwrap()
                        .unwrap()
                        .id,
                    id: auth_model_id.to_string(),
                })
                .await
                .unwrap()
                .into_inner()
                .authorization_model
                .unwrap()
        }
    }

    #[allow(clippy::unused_async)]
    async fn v1_pre_migration_fn(
        client: OpenFgaServiceClient<tonic::transport::Channel>,
        counter_mutex: Arc<Mutex<i32>>,
    ) -> std::result::Result<(), StdError> {
        let _ = client;
        // Throw an error for the second call
        let mut counter = counter_mutex.lock().unwrap();
        *counter += 1;
        if *counter == 2 {
            return Err(Box::new(Error::RequestFailed(tonic::Status::new(
                tonic::Code::Internal,
                "Test",
            ))));
        }
        Ok(())
    }

    #[allow(clippy::unused_async)]
    async fn v2_post_migration_fn(
        client: OpenFgaServiceClient<tonic::transport::Channel>,
        counter_mutex: Arc<Mutex<i32>>,
    ) -> std::result::Result<(), StdError> {
        let _ = client;
        // Throw an error for the second call
        let mut counter = counter_mutex.lock().unwrap();
        *counter += 1;
        if *counter == 2 {
            return Err(Box::new(Error::RequestFailed(tonic::Status::new(
                tonic::Code::Internal,
                "Test",
            ))));
        }
        Ok(())
    }
}
