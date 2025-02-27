use std::sync::Arc;

use tonic::codegen::StdError;

use crate::generated::ReadRequestTupleKey;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    #[error("Multiple stores with the name `{0}` found")]
    AmbiguousStoreName(String),
    #[error("Request to OpenFGA failed with status: {0}")]
    RequestFailed(tonic::Status),
    #[error("Too many pages returned for tuple {tuple}. Max pages: {max_pages}")]
    TooManyPages {
        max_pages: u32,
        tuple: ReadRequestTupleKey,
    },
    #[error("Invalid Endpoint: `{0}`")]
    InvalidEndpoint(String),
    #[error("Invalid Token: {reason}")]
    InvalidToken { reason: String },
    #[cfg(feature = "auth-middle")]
    #[error("Failed to fetch or refresh Client Credentials: {0}")]
    CredentialRefreshError(#[source] middle::Error),
    #[error(
        "Invalid OpenFGA Model Version: `{0}`. Model Versions must have the format `major.minor`"
    )]
    InvalidModelVersion(String),
    #[error("Migration Hook for model version {version} failed: {error}")]
    MigrationHookFailed {
        version: String,
        error: Arc<StdError>,
    },
    #[error("Store with Name `{0}` not found")]
    StoreNotFound(String),
    #[error("Multiple authorization models with model prefix `{model_prefix}` for version `{version}` found.")]
    AmbiguousModelVersion {
        model_prefix: String,
        version: String,
    },
    #[error(
        "Too many writes and deletes in single OpenFGA transaction (actual) {actual} > {max} (max)"
    )]
    TooManyWrites { actual: i32, max: i32 },
    #[error("Failed to write Authorization tuples: {source}")]
    WriteFailed { source: tonic::Status },
    #[error("Failed to read Authorization tuples: {source}")]
    ReadFailed { source: tonic::Status },
    #[error("Authorization check failed")]
    CheckFailed { source: tonic::Status },
}

impl Error {
    // pub fn internal(
    //     reason: impl Into<String>,
    //     error: impl Into<Box<dyn std::error::Error + Send + Sync>>,
    // ) -> Self {
    //     Self::InternalError {
    //         reason: reason.into(),
    //         source: error.into(),
    //     }
    // }
}
