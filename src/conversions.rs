use crate::client::{AuthorizationModel, WriteAuthorizationModelRequest};

impl AuthorizationModel {
    pub(crate) fn into_write_request(self, store_id: String) -> WriteAuthorizationModelRequest {
        WriteAuthorizationModelRequest {
            store_id,
            type_definitions: self.type_definitions.into_iter().collect(),
            schema_version: self.schema_version,
            conditions: self.conditions,
        }
    }
}
