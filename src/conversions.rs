use crate::client::{
    AuthorizationModel, CheckRequestTupleKey, ReadRequestTupleKey, TupleKeyWithoutCondition,
    WriteAuthorizationModelRequest,
};

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

impl From<TupleKeyWithoutCondition> for ReadRequestTupleKey {
    fn from(tuple_key: TupleKeyWithoutCondition) -> Self {
        ReadRequestTupleKey {
            user: tuple_key.user,
            relation: tuple_key.relation,
            object: tuple_key.object,
        }
    }
}

impl From<ReadRequestTupleKey> for TupleKeyWithoutCondition {
    fn from(tuple_key: ReadRequestTupleKey) -> Self {
        TupleKeyWithoutCondition {
            user: tuple_key.user,
            relation: tuple_key.relation,
            object: tuple_key.object,
        }
    }
}

impl From<CheckRequestTupleKey> for TupleKeyWithoutCondition {
    fn from(tuple_key: CheckRequestTupleKey) -> Self {
        TupleKeyWithoutCondition {
            user: tuple_key.user,
            relation: tuple_key.relation,
            object: tuple_key.object,
        }
    }
}

impl From<TupleKeyWithoutCondition> for CheckRequestTupleKey {
    fn from(tuple_key: TupleKeyWithoutCondition) -> Self {
        CheckRequestTupleKey {
            user: tuple_key.user,
            relation: tuple_key.relation,
            object: tuple_key.object,
        }
    }
}
