use crate::client::{ReadRequestTupleKey, RelationshipCondition, TupleKey};

impl std::fmt::Display for TupleKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(condition) = &self.condition {
            write!(
                f,
                "({}, {}, {}) with condition {}",
                self.user, self.relation, self.object, condition
            )
        } else {
            write!(f, "({}, {}, {})", self.user, self.relation, self.object)
        }
    }
}

impl std::fmt::Display for RelationshipCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for ReadRequestTupleKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.user, self.relation, self.object)
    }
}
