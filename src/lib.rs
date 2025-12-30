#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    clippy::pedantic
)]
#![forbid(unsafe_code)]

//! # OpenFGA Rust Client
//!
//! [![Crates.io](https://img.shields.io/crates/v/openfga-client)](https://crates.io/crates/openfga-client)
//! [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! [![Tests](https://github.com/vakamo-labs/openfga-client/actions/workflows/ci.yaml/badge.svg)](https://github.com/vakamo-labs/openfga-client/actions/workflows/ci.yaml)
//!
//! OpenFGA Rust Client is a type-safe client for OpenFGA with optional Authorization Model management and Authentication (Bearer or Client Credentials).
//!
//! ## Features
//!
//! * Type-safe client for OpenFGA (gRPC) build on `tonic`
//! * (JSON) Serialization and deserialization for Authorization Models in addition to protobuf Messages
//! * Uses `vendored-protoc` for well-known types - Rust files are pre-generated.
//! * Optional Authorization Model management with Migration hooks. Ideal for stateless deployments. State is managed exclusively in OpenFGA. This enables fully automated model management by your Application without re-writing of Authorization Models on startup.
//! * Optional Authentication (Bearer or Client Credentials) via the [Middle Crate](https://crates.io/crates/middle). (Feature: `auth-middle`)
//! * Convenience functions like `read_all_tuples` (handles pagination), `get_store_by_name` and more.
//!
//! # Usage
//!
//! ## Basic Usage
//! ```no_run
//! use openfga_client::client::OpenFgaServiceClient;
//! use tonic::transport::Channel;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let endpoint = "http://localhost:8081";
//!     let service_client = OpenFgaServiceClient::connect(endpoint).await?;
//!
//!     // Use the client to interact with OpenFGA
//!     Ok(())
//! }
//! ```
//!
//! ## Bearer Token Authentication (API-Key)
//! ```no_run
//! use openfga_client::{client::BasicOpenFgaServiceClient, url};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let endpoint = url::Url::parse("http://localhost:8081")?;
//!     let token = "your-bearer-token";
//!     let service_client = BasicOpenFgaServiceClient::new_with_basic_auth(endpoint, token)?;
//!
//!     // Use the client to interact with OpenFGA
//!     Ok(())
//! }
//! ```
//!
//! ## Client Credential Authentication
//! ```no_run
//! use openfga_client::client::BasicOpenFgaServiceClient;
//! use url::Url;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let endpoint = Url::parse("http://localhost:8081")?;
//!     let client_id = "your-client-id";
//!     let client_secret = "your-client-secret";
//!     let token_endpoint = Url::parse("http://localhost:8081/token")?;
//!     let scopes = vec!["scope1", "scope2"];
//!     let service_client = BasicOpenFgaServiceClient::new_with_client_credentials(endpoint, client_id, client_secret, token_endpoint, &scopes).await?;
//!
//!     // Use the client to interact with OpenFGA
//!     Ok(())
//! }
//! ```
//!
//! ## Authorization Model Management and Migration
//!
//! For more details please check the [`TupleModelManager`](`migration::TupleModelManager`).
//!
//! Requires the following as part of the Authorization model:
//! ```text
//! type auth_model_id
//! type model_version
//!   relations
//!     define openfga_id: [auth_model_id]
//!     define exists: [auth_model_id:*]
//! ```
//!
//! Usage:
//! ```no_run
//! use openfga_client::client::{OpenFgaServiceClient, TupleKeyWithoutCondition};
//! use openfga_client::migration::{AuthorizationModelVersion, MigrationFn, TupleModelManager};
//! use openfga_client::tonic::codegen::StdError;
//!
//! /// Application specific state passed into migration functions.
//! ///
//! /// It must be clone so that in can be passed into *both* pre and post migration hooks.
//! #[derive(Clone)]
//! struct MyMigrationState {}
//!
//! /// An example MigrationFn.
//! #[allow(clippy::unused_async)]
//! async fn v1_1_migration(
//!     _client: OpenFgaServiceClient<tonic::transport::Channel>,
//!     _prev_auth_model_id: Option<String>,
//!     _active_auth_model_id: Option<String>,
//!     _state: MyMigrationState,
//! ) -> std::result::Result<(), StdError> {
//!     // `client` and `state` can be used to read and write tuples from the store
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let endpoint = "http://localhost:8081";
//!     let mut service_client = OpenFgaServiceClient::connect(endpoint).await?;
//!
//!     let store_name = "my-store";
//!     let model_prefix = "my-model";
//!
//!     let mut manager = TupleModelManager::new(service_client.clone(), store_name, model_prefix)
//!         // Migrations are executed in order for models that have not been previously migrated.
//!         // First model - version 1.0
//!         .add_model(
//!             serde_json::from_str(include_str!("../tests/model-manager/v1.0/schema.json"))?,
//!             AuthorizationModelVersion::new(1, 0),
//!             // For major version upgrades, this is where tuple migrations go.
//!             None::<MigrationFn<_, _>>,
//!             None::<MigrationFn<_, _>>,
//!         )
//!         // Second model - version 1.1
//!         .add_model(
//!             serde_json::from_str(include_str!("../tests/model-manager/v1.1/schema.json"))?,
//!             AuthorizationModelVersion::new(1, 1),
//!             // For major version upgrades, this is where tuple migrations go.
//!             Some(v1_1_migration),
//!             None::<MigrationFn<_, _>>,
//!         );
//!
//!     // Perform the migration if necessary
//!     manager.migrate(MyMigrationState {}).await?;
//!
//!     let store_id = service_client
//!         .get_store_by_name(store_name)
//!         .await?
//!         .expect("Store found")
//!         .id;
//!     let authorization_model_id = manager
//!         .get_authorization_model_id(AuthorizationModelVersion::new(1, 1))
//!         .await?
//!         .expect("Authorization model found");
//!     let client = service_client.into_client(&store_id, &authorization_model_id);
//!
//!     // Use the client.
//!     // `store_id` and `authorization_model_id` are stored in the client and attached to all requests.
//!     let page_size = 100;
//!     let continuation_token = None;
//!     let _tuples = client
//!         .read(
//!             page_size,
//!             TupleKeyWithoutCondition {
//!                 user: "user:peter".to_string(),
//!                 relation: "owner".to_string(),
//!                 object: "organization:my-org".to_string(),
//!             },
//!             continuation_token,
//!         )
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## License
//! This project is licensed under the Apache-2.0 License. See the LICENSE file for details.
//!
//! ## Contributing
//! Contributions are welcome! Please open an issue or submit a pull request on GitHub.

pub use prost_types;
pub use prost_wkt_types;
pub use tonic;
pub mod display;
pub mod error;
pub mod migration;
pub use url;

mod client_ext;
mod conversions;
mod model_client;

mod generated {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]

    include!("gen/openfga/v1/openfga.v1.rs");
}

pub mod client {
    //! Contains clients to connect to OpenFGA:
    //!
    //! * [`OpenFgaServiceClient`] is the generated client that allows full control over all parameters.
    //! * [`OpenFgaClient`] is a wrapper around the generated client, that provides a more convenient interface and adds `store_id`, `authorization_model_id` and `consistency` to all requests.
    //!
    pub use open_fga_service_client::OpenFgaServiceClient;

    #[cfg(feature = "auth-middle")]
    pub use super::client_ext::{BasicAuthLayer, BasicOpenFgaServiceClient};
    #[cfg(feature = "auth-middle")]
    pub use super::model_client::BasicOpenFgaClient;
    pub use super::{
        generated::*,
        model_client::{ConflictBehavior, OpenFgaClient, WriteOptions},
    };
}

#[cfg(test)]
mod test_json_serde {
    use super::client::*;

    fn test_authorization_model_serde(schema: &str) {
        let schema_json: serde_json::Value = schema.parse().unwrap();
        let schema: AuthorizationModel = serde_json::from_value(schema_json.clone()).unwrap();
        let value = serde_json::to_value(&schema).unwrap();
        assert_eq!(schema_json, value);
    }
    #[test]
    fn test_serde_custom_roles() {
        test_authorization_model_serde(include_str!(
            "../tests/sample-store/custom-roles/schema.json"
        ));
    }

    #[test]
    fn test_serde_entitlements() {
        test_authorization_model_serde(include_str!(
            "../tests/sample-store/entitlements/schema.json"
        ));
    }

    #[test]
    fn test_serde_expenses() {
        test_authorization_model_serde(include_str!("../tests/sample-store/expenses/schema.json"));
    }

    // gdrive, github, iot, issue-tracker, modular, slack
    #[test]
    fn test_serde_gdrive() {
        test_authorization_model_serde(include_str!("../tests/sample-store/gdrive/schema.json"));
    }

    #[test]
    fn test_serde_github() {
        test_authorization_model_serde(include_str!("../tests/sample-store/github/schema.json"));
    }

    #[test]
    fn test_serde_iot() {
        test_authorization_model_serde(include_str!("../tests/sample-store/iot/schema.json"));
    }

    #[test]
    fn test_serde_modular() {
        test_authorization_model_serde(include_str!("../tests/sample-store/modular/schema.json"));
    }

    #[test]
    fn test_serde_slack() {
        test_authorization_model_serde(include_str!("../tests/sample-store/slack/schema.json"));
    }
}
