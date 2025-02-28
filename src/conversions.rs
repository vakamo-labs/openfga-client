use crate::client::{
    AuthorizationModel, CheckRequestTupleKey, ContextualTupleKeys, ReadRequestTupleKey, TupleKey,
    TupleKeyWithoutCondition, WriteAuthorizationModelRequest,
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

impl From<TupleKeyWithoutCondition> for Option<ReadRequestTupleKey> {
    fn from(tuple_key: TupleKeyWithoutCondition) -> Self {
        Some(tuple_key.into())
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

impl From<CheckRequestTupleKey> for Option<TupleKeyWithoutCondition> {
    fn from(tuple_key: CheckRequestTupleKey) -> Self {
        Some(tuple_key.into())
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

impl From<TupleKeyWithoutCondition> for Option<CheckRequestTupleKey> {
    fn from(tuple_key: TupleKeyWithoutCondition) -> Self {
        Some(tuple_key.into())
    }
}

impl From<ContextualTupleKeys> for Vec<TupleKey> {
    fn from(tuple_keys: ContextualTupleKeys) -> Self {
        tuple_keys.tuple_keys
    }
}

impl From<Vec<TupleKey>> for ContextualTupleKeys {
    fn from(tuple_keys: Vec<TupleKey>) -> Self {
        ContextualTupleKeys { tuple_keys }
    }
}

impl From<ContextualTupleKeys> for Option<Vec<TupleKey>> {
    fn from(tuple_keys: ContextualTupleKeys) -> Self {
        if tuple_keys.tuple_keys.is_empty() {
            None
        } else {
            Some(tuple_keys.tuple_keys)
        }
    }
}
