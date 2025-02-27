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
//! * No dependency on `protoc` - Rust files are pre-generated.
//! * Type-safe client for OpenFGA (gRPC) build on `tonic`
//! * (JSON) Serialization and deserialization for Authorization Models in addition go protobuf Messages
//! * Optional Authorization Model management with Migration hooks if tuples need to be re-written. Ideal for stateless deployments. State is managed exclusively in OpenFGA. This enables fully automated model management by your Application without blindly re-writing of Authorization Models on startup!
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
//!     let endpoint = "http://localhost:8080";
//!     let client = OpenFgaServiceClient::connect(endpoint).await?;
//!
//!     // Use the client to interact with OpenFGA
//!     Ok(())
//! }
//! ```
//!
//! ## Bearer Token Authentication (API-Key)
//! ```no_run
//! use openfga_client::client::BasicOpenFgaServiceClient;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let endpoint = "http://localhost:8080";
//!     let token = "your-bearer-token";
//!     let client = BasicOpenFgaServiceClient::new_with_basic_auth(endpoint, token)?;
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
//!     let endpoint = "http://localhost:8080";
//!     let client_id = "your-client-id";
//!     let client_secret = "your-client-secret";
//!     let token_endpoint = Url::parse("http://localhost:8080/token")?;
//!     let client = BasicOpenFgaServiceClient::new_with_client_credentials(endpoint, client_id, client_secret, &token_endpoint).await?;
//!
//!     // Use the client to interact with OpenFGA
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

mod client_ext;
mod conversions;
mod model_client;

mod generated {
    #![allow(clippy::all)]
    #![allow(clippy::pedantic)]

    include!("gen/openfga.v1.rs");
}

pub mod client {
    pub use open_fga_service_client::OpenFgaServiceClient;

    #[cfg(feature = "auth-middle")]
    pub use super::client_ext::BasicOpenFgaServiceClient;
    pub use super::generated::*;

    pub use super::model_client::OpenFgaClient;
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
