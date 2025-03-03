# OpenFGA Rust Client

[![Crates.io](https://img.shields.io/crates/v/openfga-client)](https://crates.io/crates/openfga-client)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Tests](https://github.com/vakamo-labs/openfga-client/actions/workflows/ci.yaml/badge.svg)](https://github.com/vakamo-labs/openfga-client/actions/workflows/ci.yaml)

OpenFGA Rust Client is a type-safe gRPC client for OpenFGA with optional Authorization Model management and Authentication (Bearer or Client Credentials).

# Features

* Type-safe client for OpenFGA (gRPC) build on `tonic`
* (JSON) Serialization and deserialization for Authorization Models in addition go protobuf Messages
* Uses `vendored-protoc` for well-known types - Rust files are pre-generated.
* Optional Authorization Model management with Migration hooks. Ideal for stateless deployments. State is managed exclusively in OpenFGA. This enables fully automated model management by your Application without re-writing of Authorization Models on startup.
* Optional Authentication (Bearer or Client Credentials) via the [Middle Crate](https://crates.io/crates/middle). (Feature: `auth-middle`)
* Convenience functions like `read_all_tuples` (handles pagination), `get_store_by_name` and more.

# Usage

## Basic Usage
```rust
use openfga_client::client::OpenFgaServiceClient;
use tonic::transport::Channel;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:8081";
    let service_client = OpenFgaServiceClient::connect(endpoint).await?;
    // Use the client to interact with OpenFGA
    Ok(())
}
```

## Bearer Token Authentication (API-Key)
```rust
use openfga_client::{client::BasicOpenFgaServiceClient, url};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = url::Url::parse("http://localhost:8081")?;
    let token = "your-bearer-token";
    let service_client = BasicOpenFgaServiceClient::new_with_basic_auth(endpoint, token)?;
    // Use the client to interact with OpenFGA
    Ok(())
}
```

## Client Credential Authentication
```rust
use openfga_client::client::BasicOpenFgaServiceClient;
use url::Url;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("http://localhost:8081")?;
    let client_id = "your-client-id";
    let client_secret = "your-client-secret";
    let token_endpoint = Url::parse("http://localhost:8081/token")?;
    let scopes = vec!["scope1", "scope2"];
    let service_client = BasicOpenFgaServiceClient::new_with_client_credentials(endpoint, client_id, client_secret, token_endpoint, &scopes).await?;
    // Use the client to interact with OpenFGA
    Ok(())
}
```

# License
This project is licensed under the Apache-2.0 License. See the LICENSE file for details.

# Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.
