// @generated
impl serde::Serialize for AbortedMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.AbortedMessageResponse", len)?;
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AbortedMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbortedMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.AbortedMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AbortedMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AbortedMessageResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.AbortedMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Assertion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tuple_key.is_some() {
            len += 1;
        }
        if self.expectation {
            len += 1;
        }
        if !self.contextual_tuples.is_empty() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Assertion", len)?;
        if let Some(v) = self.tuple_key.as_ref() {
            struct_ser.serialize_field("tuple_key", v)?;
        }
        if self.expectation {
            struct_ser.serialize_field("expectation", &self.expectation)?;
        }
        if !self.contextual_tuples.is_empty() {
            struct_ser.serialize_field("contextual_tuples", &self.contextual_tuples)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Assertion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuple_key",
            "expectation",
            "contextual_tuples",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TupleKey,
            Expectation,
            ContextualTuples,
            Context,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuple_key" => Ok(GeneratedField::TupleKey),
                            "expectation" => Ok(GeneratedField::Expectation),
                            "contextual_tuples" => Ok(GeneratedField::ContextualTuples),
                            "context" => Ok(GeneratedField::Context),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Assertion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Assertion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Assertion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuple_key__ = None;
                let mut expectation__ = None;
                let mut contextual_tuples__ = None;
                let mut context__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TupleKey => {
                            if tuple_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_key"));
                            }
                            tuple_key__ = map_.next_value()?;
                        }
                        GeneratedField::Expectation => {
                            if expectation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectation"));
                            }
                            expectation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextualTuples => {
                            if contextual_tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextual_tuples"));
                            }
                            contextual_tuples__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Assertion {
                    tuple_key: tuple_key__,
                    expectation: expectation__.unwrap_or_default(),
                    contextual_tuples: contextual_tuples__.unwrap_or_default(),
                    context: context__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Assertion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AssertionTupleKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.AssertionTupleKey", len)?;
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssertionTupleKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object",
            "relation",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Object,
            Relation,
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "object" => Ok(GeneratedField::Object),
                            "relation" => Ok(GeneratedField::Relation),
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssertionTupleKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.AssertionTupleKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssertionTupleKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object__ = None;
                let mut relation__ = None;
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AssertionTupleKey {
                    object: object__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.AssertionTupleKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Assertions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assertions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Assertions", len)?;
        if !self.assertions.is_empty() {
            struct_ser.serialize_field("assertions", &self.assertions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Assertions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assertions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assertions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assertions" => Ok(GeneratedField::Assertions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Assertions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Assertions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Assertions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assertions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assertions => {
                            if assertions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertions"));
                            }
                            assertions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Assertions {
                    assertions: assertions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Assertions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoAuthError => "no_auth_error",
            Self::AuthFailedInvalidSubject => "auth_failed_invalid_subject",
            Self::AuthFailedInvalidAudience => "auth_failed_invalid_audience",
            Self::AuthFailedInvalidIssuer => "auth_failed_invalid_issuer",
            Self::InvalidClaims => "invalid_claims",
            Self::AuthFailedInvalidBearerToken => "auth_failed_invalid_bearer_token",
            Self::BearerTokenMissing => "bearer_token_missing",
            Self::Unauthenticated => "unauthenticated",
            Self::Forbidden => "forbidden",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AuthErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "no_auth_error",
            "auth_failed_invalid_subject",
            "auth_failed_invalid_audience",
            "auth_failed_invalid_issuer",
            "invalid_claims",
            "auth_failed_invalid_bearer_token",
            "bearer_token_missing",
            "unauthenticated",
            "forbidden",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "no_auth_error" => Ok(AuthErrorCode::NoAuthError),
                    "auth_failed_invalid_subject" => Ok(AuthErrorCode::AuthFailedInvalidSubject),
                    "auth_failed_invalid_audience" => Ok(AuthErrorCode::AuthFailedInvalidAudience),
                    "auth_failed_invalid_issuer" => Ok(AuthErrorCode::AuthFailedInvalidIssuer),
                    "invalid_claims" => Ok(AuthErrorCode::InvalidClaims),
                    "auth_failed_invalid_bearer_token" => Ok(AuthErrorCode::AuthFailedInvalidBearerToken),
                    "bearer_token_missing" => Ok(AuthErrorCode::BearerTokenMissing),
                    "unauthenticated" => Ok(AuthErrorCode::Unauthenticated),
                    "forbidden" => Ok(AuthErrorCode::Forbidden),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizationModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.schema_version.is_empty() {
            len += 1;
        }
        if !self.type_definitions.is_empty() {
            len += 1;
        }
        if !self.conditions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.AuthorizationModel", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.schema_version.is_empty() {
            struct_ser.serialize_field("schema_version", &self.schema_version)?;
        }
        if !self.type_definitions.is_empty() {
            struct_ser.serialize_field("type_definitions", &self.type_definitions)?;
        }
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "schema_version",
            "type_definitions",
            "conditions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            SchemaVersion,
            TypeDefinitions,
            Conditions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "schema_version" => Ok(GeneratedField::SchemaVersion),
                            "type_definitions" => Ok(GeneratedField::TypeDefinitions),
                            "conditions" => Ok(GeneratedField::Conditions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.AuthorizationModel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AuthorizationModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut schema_version__ = None;
                let mut type_definitions__ = None;
                let mut conditions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SchemaVersion => {
                            if schema_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema_version"));
                            }
                            schema_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeDefinitions => {
                            if type_definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type_definitions"));
                            }
                            type_definitions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(AuthorizationModel {
                    id: id__.unwrap_or_default(),
                    schema_version: schema_version__.unwrap_or_default(),
                    type_definitions: type_definitions__.unwrap_or_default(),
                    conditions: conditions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.AuthorizationModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCheckItem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tuple_key.is_some() {
            len += 1;
        }
        if self.contextual_tuples.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if !self.correlation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.BatchCheckItem", len)?;
        if let Some(v) = self.tuple_key.as_ref() {
            struct_ser.serialize_field("tuple_key", v)?;
        }
        if let Some(v) = self.contextual_tuples.as_ref() {
            struct_ser.serialize_field("contextual_tuples", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if !self.correlation_id.is_empty() {
            struct_ser.serialize_field("correlation_id", &self.correlation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCheckItem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuple_key",
            "contextual_tuples",
            "context",
            "correlation_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TupleKey,
            ContextualTuples,
            Context,
            CorrelationId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuple_key" => Ok(GeneratedField::TupleKey),
                            "contextual_tuples" => Ok(GeneratedField::ContextualTuples),
                            "context" => Ok(GeneratedField::Context),
                            "correlation_id" => Ok(GeneratedField::CorrelationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCheckItem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.BatchCheckItem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCheckItem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuple_key__ = None;
                let mut contextual_tuples__ = None;
                let mut context__ = None;
                let mut correlation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TupleKey => {
                            if tuple_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_key"));
                            }
                            tuple_key__ = map_.next_value()?;
                        }
                        GeneratedField::ContextualTuples => {
                            if contextual_tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextual_tuples"));
                            }
                            contextual_tuples__ = map_.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                        GeneratedField::CorrelationId => {
                            if correlation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("correlation_id"));
                            }
                            correlation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchCheckItem {
                    tuple_key: tuple_key__,
                    contextual_tuples: contextual_tuples__,
                    context: context__,
                    correlation_id: correlation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.BatchCheckItem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.checks.is_empty() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if self.consistency != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.BatchCheckRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.checks.is_empty() {
            struct_ser.serialize_field("checks", &self.checks)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if self.consistency != 0 {
            let v = ConsistencyPreference::try_from(self.consistency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.consistency)))?;
            struct_ser.serialize_field("consistency", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "checks",
            "authorization_model_id",
            "consistency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            Checks,
            AuthorizationModelId,
            Consistency,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "checks" => Ok(GeneratedField::Checks),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "consistency" => Ok(GeneratedField::Consistency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.BatchCheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut checks__ = None;
                let mut authorization_model_id__ = None;
                let mut consistency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Checks => {
                            if checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checks"));
                            }
                            checks__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = Some(map_.next_value::<ConsistencyPreference>()? as i32);
                        }
                    }
                }
                Ok(BatchCheckRequest {
                    store_id: store_id__.unwrap_or_default(),
                    checks: checks__.unwrap_or_default(),
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    consistency: consistency__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.BatchCheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.result.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.BatchCheckResponse", len)?;
        if !self.result.is_empty() {
            struct_ser.serialize_field("result", &self.result)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.BatchCheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(BatchCheckResponse {
                    result: result__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.BatchCheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchCheckSingleResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.check_result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.BatchCheckSingleResult", len)?;
        if let Some(v) = self.check_result.as_ref() {
            match v {
                batch_check_single_result::CheckResult::Allowed(v) => {
                    struct_ser.serialize_field("allowed", v)?;
                }
                batch_check_single_result::CheckResult::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchCheckSingleResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Allowed,
            Error,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "allowed" => Ok(GeneratedField::Allowed),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchCheckSingleResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.BatchCheckSingleResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchCheckSingleResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut check_result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Allowed => {
                            if check_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowed"));
                            }
                            check_result__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_check_single_result::CheckResult::Allowed);
                        }
                        GeneratedField::Error => {
                            if check_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            check_result__ = map_.next_value::<::std::option::Option<_>>()?.map(batch_check_single_result::CheckResult::Error)
;
                        }
                    }
                }
                Ok(BatchCheckSingleResult {
                    check_result: check_result__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.BatchCheckSingleResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.CheckError", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if let Some(v) = self.code.as_ref() {
            match v {
                check_error::Code::InputError(v) => {
                    let v = ErrorCode::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("input_error", &v)?;
                }
                check_error::Code::InternalError(v) => {
                    let v = InternalErrorCode::try_from(*v)
                        .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("internal_error", &v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
            "input_error",
            "internal_error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            InputError,
            InternalError,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            "input_error" => Ok(GeneratedField::InputError),
                            "internal_error" => Ok(GeneratedField::InternalError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.CheckError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputError => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("input_error"));
                            }
                            code__ = map_.next_value::<::std::option::Option<ErrorCode>>()?.map(|x| check_error::Code::InputError(x as i32));
                        }
                        GeneratedField::InternalError => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internal_error"));
                            }
                            code__ = map_.next_value::<::std::option::Option<InternalErrorCode>>()?.map(|x| check_error::Code::InternalError(x as i32));
                        }
                    }
                }
                Ok(CheckError {
                    message: message__.unwrap_or_default(),
                    code: code__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.CheckError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if self.tuple_key.is_some() {
            len += 1;
        }
        if self.contextual_tuples.is_some() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if self.trace {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.consistency != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.CheckRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if let Some(v) = self.tuple_key.as_ref() {
            struct_ser.serialize_field("tuple_key", v)?;
        }
        if let Some(v) = self.contextual_tuples.as_ref() {
            struct_ser.serialize_field("contextual_tuples", v)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if self.trace {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if self.consistency != 0 {
            let v = ConsistencyPreference::try_from(self.consistency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.consistency)))?;
            struct_ser.serialize_field("consistency", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "tuple_key",
            "contextual_tuples",
            "authorization_model_id",
            "trace",
            "context",
            "consistency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            TupleKey,
            ContextualTuples,
            AuthorizationModelId,
            Trace,
            Context,
            Consistency,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "tuple_key" => Ok(GeneratedField::TupleKey),
                            "contextual_tuples" => Ok(GeneratedField::ContextualTuples),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "trace" => Ok(GeneratedField::Trace),
                            "context" => Ok(GeneratedField::Context),
                            "consistency" => Ok(GeneratedField::Consistency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.CheckRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut tuple_key__ = None;
                let mut contextual_tuples__ = None;
                let mut authorization_model_id__ = None;
                let mut trace__ = None;
                let mut context__ = None;
                let mut consistency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TupleKey => {
                            if tuple_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_key"));
                            }
                            tuple_key__ = map_.next_value()?;
                        }
                        GeneratedField::ContextualTuples => {
                            if contextual_tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextual_tuples"));
                            }
                            contextual_tuples__ = map_.next_value()?;
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = Some(map_.next_value::<ConsistencyPreference>()? as i32);
                        }
                    }
                }
                Ok(CheckRequest {
                    store_id: store_id__.unwrap_or_default(),
                    tuple_key: tuple_key__,
                    contextual_tuples: contextual_tuples__,
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    trace: trace__.unwrap_or_default(),
                    context: context__,
                    consistency: consistency__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.CheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckRequestTupleKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.object.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.CheckRequestTupleKey", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckRequestTupleKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "relation",
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Relation,
            Object,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "relation" => Ok(GeneratedField::Relation),
                            "object" => Ok(GeneratedField::Object),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckRequestTupleKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.CheckRequestTupleKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckRequestTupleKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckRequestTupleKey {
                    user: user__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    object: object__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.CheckRequestTupleKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed {
            len += 1;
        }
        if !self.resolution.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.CheckResponse", len)?;
        if self.allowed {
            struct_ser.serialize_field("allowed", &self.allowed)?;
        }
        if !self.resolution.is_empty() {
            struct_ser.serialize_field("resolution", &self.resolution)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed",
            "resolution",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Allowed,
            Resolution,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "allowed" => Ok(GeneratedField::Allowed),
                            "resolution" => Ok(GeneratedField::Resolution),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.CheckResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed__ = None;
                let mut resolution__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Allowed => {
                            if allowed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowed"));
                            }
                            allowed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resolution => {
                            if resolution__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolution"));
                            }
                            resolution__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CheckResponse {
                    allowed: allowed__.unwrap_or_default(),
                    resolution: resolution__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.CheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ComputedUserset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ComputedUserset", len)?;
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ComputedUserset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ComputedUserset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ComputedUserset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ComputedUserset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ComputedUserset {
                    relation: relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ComputedUserset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Condition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.expression.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Condition", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.expression.is_empty() {
            struct_ser.serialize_field("expression", &self.expression)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Condition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "expression",
            "parameters",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Expression,
            Parameters,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "expression" => Ok(GeneratedField::Expression),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Condition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Condition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Condition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut expression__ = None;
                let mut parameters__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expression => {
                            if expression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expression"));
                            }
                            expression__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Condition {
                    name: name__.unwrap_or_default(),
                    expression: expression__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Condition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConditionMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.module.is_empty() {
            len += 1;
        }
        if self.source_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ConditionMetadata", len)?;
        if !self.module.is_empty() {
            struct_ser.serialize_field("module", &self.module)?;
        }
        if let Some(v) = self.source_info.as_ref() {
            struct_ser.serialize_field("source_info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConditionMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "module",
            "source_info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Module,
            SourceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "module" => Ok(GeneratedField::Module),
                            "source_info" => Ok(GeneratedField::SourceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConditionMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ConditionMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConditionMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut module__ = None;
                let mut source_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Module => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module"));
                            }
                            module__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceInfo => {
                            if source_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source_info"));
                            }
                            source_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ConditionMetadata {
                    module: module__.unwrap_or_default(),
                    source_info: source_info__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ConditionMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConditionParamTypeRef {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.type_name != 0 {
            len += 1;
        }
        if !self.generic_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ConditionParamTypeRef", len)?;
        if self.type_name != 0 {
            let v = condition_param_type_ref::TypeName::try_from(self.type_name)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.type_name)))?;
            struct_ser.serialize_field("type_name", &v)?;
        }
        if !self.generic_types.is_empty() {
            struct_ser.serialize_field("generic_types", &self.generic_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConditionParamTypeRef {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type_name",
            "generic_types",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeName,
            GenericTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type_name" => Ok(GeneratedField::TypeName),
                            "generic_types" => Ok(GeneratedField::GenericTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConditionParamTypeRef;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ConditionParamTypeRef")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConditionParamTypeRef, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_name__ = None;
                let mut generic_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypeName => {
                            if type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type_name"));
                            }
                            type_name__ = Some(map_.next_value::<condition_param_type_ref::TypeName>()? as i32);
                        }
                        GeneratedField::GenericTypes => {
                            if generic_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generic_types"));
                            }
                            generic_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ConditionParamTypeRef {
                    type_name: type_name__.unwrap_or_default(),
                    generic_types: generic_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ConditionParamTypeRef", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for condition_param_type_ref::TypeName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TYPE_NAME_UNSPECIFIED",
            Self::Any => "TYPE_NAME_ANY",
            Self::Bool => "TYPE_NAME_BOOL",
            Self::String => "TYPE_NAME_STRING",
            Self::Int => "TYPE_NAME_INT",
            Self::Uint => "TYPE_NAME_UINT",
            Self::Double => "TYPE_NAME_DOUBLE",
            Self::Duration => "TYPE_NAME_DURATION",
            Self::Timestamp => "TYPE_NAME_TIMESTAMP",
            Self::Map => "TYPE_NAME_MAP",
            Self::List => "TYPE_NAME_LIST",
            Self::Ipaddress => "TYPE_NAME_IPADDRESS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for condition_param_type_ref::TypeName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TYPE_NAME_UNSPECIFIED",
            "TYPE_NAME_ANY",
            "TYPE_NAME_BOOL",
            "TYPE_NAME_STRING",
            "TYPE_NAME_INT",
            "TYPE_NAME_UINT",
            "TYPE_NAME_DOUBLE",
            "TYPE_NAME_DURATION",
            "TYPE_NAME_TIMESTAMP",
            "TYPE_NAME_MAP",
            "TYPE_NAME_LIST",
            "TYPE_NAME_IPADDRESS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = condition_param_type_ref::TypeName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TYPE_NAME_UNSPECIFIED" => Ok(condition_param_type_ref::TypeName::Unspecified),
                    "TYPE_NAME_ANY" => Ok(condition_param_type_ref::TypeName::Any),
                    "TYPE_NAME_BOOL" => Ok(condition_param_type_ref::TypeName::Bool),
                    "TYPE_NAME_STRING" => Ok(condition_param_type_ref::TypeName::String),
                    "TYPE_NAME_INT" => Ok(condition_param_type_ref::TypeName::Int),
                    "TYPE_NAME_UINT" => Ok(condition_param_type_ref::TypeName::Uint),
                    "TYPE_NAME_DOUBLE" => Ok(condition_param_type_ref::TypeName::Double),
                    "TYPE_NAME_DURATION" => Ok(condition_param_type_ref::TypeName::Duration),
                    "TYPE_NAME_TIMESTAMP" => Ok(condition_param_type_ref::TypeName::Timestamp),
                    "TYPE_NAME_MAP" => Ok(condition_param_type_ref::TypeName::Map),
                    "TYPE_NAME_LIST" => Ok(condition_param_type_ref::TypeName::List),
                    "TYPE_NAME_IPADDRESS" => Ok(condition_param_type_ref::TypeName::Ipaddress),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConsistencyPreference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::MinimizeLatency => "MINIMIZE_LATENCY",
            Self::HigherConsistency => "HIGHER_CONSISTENCY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConsistencyPreference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "MINIMIZE_LATENCY",
            "HIGHER_CONSISTENCY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsistencyPreference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(ConsistencyPreference::Unspecified),
                    "MINIMIZE_LATENCY" => Ok(ConsistencyPreference::MinimizeLatency),
                    "HIGHER_CONSISTENCY" => Ok(ConsistencyPreference::HigherConsistency),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ContextualTupleKeys {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tuple_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ContextualTupleKeys", len)?;
        if !self.tuple_keys.is_empty() {
            struct_ser.serialize_field("tuple_keys", &self.tuple_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContextualTupleKeys {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuple_keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TupleKeys,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuple_keys" => Ok(GeneratedField::TupleKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ContextualTupleKeys;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ContextualTupleKeys")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ContextualTupleKeys, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuple_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TupleKeys => {
                            if tuple_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_keys"));
                            }
                            tuple_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ContextualTupleKeys {
                    tuple_keys: tuple_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ContextualTupleKeys", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateStoreRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.CreateStoreRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateStoreRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateStoreRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.CreateStoreRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateStoreRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateStoreRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.CreateStoreRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateStoreResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.CreateStoreResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("created_at", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updated_at", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateStoreResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "created_at",
            "updated_at",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "created_at" => Ok(GeneratedField::CreatedAt),
                            "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateStoreResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.CreateStoreResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateStoreResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated_at"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateStoreResponse {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.CreateStoreResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteStoreRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.DeleteStoreRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteStoreRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteStoreRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.DeleteStoreRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteStoreRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeleteStoreRequest {
                    store_id: store_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.DeleteStoreRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteStoreResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("openfga.v1.DeleteStoreResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteStoreResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteStoreResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.DeleteStoreResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteStoreResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeleteStoreResponse {
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.DeleteStoreResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Difference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base.is_some() {
            len += 1;
        }
        if self.subtract.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Difference", len)?;
        if let Some(v) = self.base.as_ref() {
            struct_ser.serialize_field("base", v)?;
        }
        if let Some(v) = self.subtract.as_ref() {
            struct_ser.serialize_field("subtract", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Difference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base",
            "subtract",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Base,
            Subtract,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "base" => Ok(GeneratedField::Base),
                            "subtract" => Ok(GeneratedField::Subtract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Difference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Difference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Difference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base__ = None;
                let mut subtract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = map_.next_value()?;
                        }
                        GeneratedField::Subtract => {
                            if subtract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subtract"));
                            }
                            subtract__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Difference {
                    base: base__,
                    subtract: subtract__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Difference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DirectUserset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("openfga.v1.DirectUserset", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DirectUserset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DirectUserset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.DirectUserset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DirectUserset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DirectUserset {
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.DirectUserset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoError => "no_error",
            Self::ValidationError => "validation_error",
            Self::AuthorizationModelNotFound => "authorization_model_not_found",
            Self::AuthorizationModelResolutionTooComplex => "authorization_model_resolution_too_complex",
            Self::InvalidWriteInput => "invalid_write_input",
            Self::CannotAllowDuplicateTuplesInOneRequest => "cannot_allow_duplicate_tuples_in_one_request",
            Self::CannotAllowDuplicateTypesInOneRequest => "cannot_allow_duplicate_types_in_one_request",
            Self::CannotAllowMultipleReferencesToOneRelation => "cannot_allow_multiple_references_to_one_relation",
            Self::InvalidContinuationToken => "invalid_continuation_token",
            Self::InvalidTupleSet => "invalid_tuple_set",
            Self::InvalidCheckInput => "invalid_check_input",
            Self::InvalidExpandInput => "invalid_expand_input",
            Self::UnsupportedUserSet => "unsupported_user_set",
            Self::InvalidObjectFormat => "invalid_object_format",
            Self::WriteFailedDueToInvalidInput => "write_failed_due_to_invalid_input",
            Self::AuthorizationModelAssertionsNotFound => "authorization_model_assertions_not_found",
            Self::LatestAuthorizationModelNotFound => "latest_authorization_model_not_found",
            Self::TypeNotFound => "type_not_found",
            Self::RelationNotFound => "relation_not_found",
            Self::EmptyRelationDefinition => "empty_relation_definition",
            Self::InvalidUser => "invalid_user",
            Self::InvalidTuple => "invalid_tuple",
            Self::UnknownRelation => "unknown_relation",
            Self::StoreIdInvalidLength => "store_id_invalid_length",
            Self::AssertionsTooManyItems => "assertions_too_many_items",
            Self::IdTooLong => "id_too_long",
            Self::AuthorizationModelIdTooLong => "authorization_model_id_too_long",
            Self::TupleKeyValueNotSpecified => "tuple_key_value_not_specified",
            Self::TupleKeysTooManyOrTooFewItems => "tuple_keys_too_many_or_too_few_items",
            Self::PageSizeInvalid => "page_size_invalid",
            Self::ParamMissingValue => "param_missing_value",
            Self::DifferenceBaseMissingValue => "difference_base_missing_value",
            Self::SubtractBaseMissingValue => "subtract_base_missing_value",
            Self::ObjectTooLong => "object_too_long",
            Self::RelationTooLong => "relation_too_long",
            Self::TypeDefinitionsTooFewItems => "type_definitions_too_few_items",
            Self::TypeInvalidLength => "type_invalid_length",
            Self::TypeInvalidPattern => "type_invalid_pattern",
            Self::RelationsTooFewItems => "relations_too_few_items",
            Self::RelationsTooLong => "relations_too_long",
            Self::RelationsInvalidPattern => "relations_invalid_pattern",
            Self::ObjectInvalidPattern => "object_invalid_pattern",
            Self::QueryStringTypeContinuationTokenMismatch => "query_string_type_continuation_token_mismatch",
            Self::ExceededEntityLimit => "exceeded_entity_limit",
            Self::InvalidContextualTuple => "invalid_contextual_tuple",
            Self::DuplicateContextualTuple => "duplicate_contextual_tuple",
            Self::InvalidAuthorizationModel => "invalid_authorization_model",
            Self::UnsupportedSchemaVersion => "unsupported_schema_version",
            Self::Cancelled => "cancelled",
            Self::InvalidStartTime => "invalid_start_time",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "no_error",
            "validation_error",
            "authorization_model_not_found",
            "authorization_model_resolution_too_complex",
            "invalid_write_input",
            "cannot_allow_duplicate_tuples_in_one_request",
            "cannot_allow_duplicate_types_in_one_request",
            "cannot_allow_multiple_references_to_one_relation",
            "invalid_continuation_token",
            "invalid_tuple_set",
            "invalid_check_input",
            "invalid_expand_input",
            "unsupported_user_set",
            "invalid_object_format",
            "write_failed_due_to_invalid_input",
            "authorization_model_assertions_not_found",
            "latest_authorization_model_not_found",
            "type_not_found",
            "relation_not_found",
            "empty_relation_definition",
            "invalid_user",
            "invalid_tuple",
            "unknown_relation",
            "store_id_invalid_length",
            "assertions_too_many_items",
            "id_too_long",
            "authorization_model_id_too_long",
            "tuple_key_value_not_specified",
            "tuple_keys_too_many_or_too_few_items",
            "page_size_invalid",
            "param_missing_value",
            "difference_base_missing_value",
            "subtract_base_missing_value",
            "object_too_long",
            "relation_too_long",
            "type_definitions_too_few_items",
            "type_invalid_length",
            "type_invalid_pattern",
            "relations_too_few_items",
            "relations_too_long",
            "relations_invalid_pattern",
            "object_invalid_pattern",
            "query_string_type_continuation_token_mismatch",
            "exceeded_entity_limit",
            "invalid_contextual_tuple",
            "duplicate_contextual_tuple",
            "invalid_authorization_model",
            "unsupported_schema_version",
            "cancelled",
            "invalid_start_time",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "no_error" => Ok(ErrorCode::NoError),
                    "validation_error" => Ok(ErrorCode::ValidationError),
                    "authorization_model_not_found" => Ok(ErrorCode::AuthorizationModelNotFound),
                    "authorization_model_resolution_too_complex" => Ok(ErrorCode::AuthorizationModelResolutionTooComplex),
                    "invalid_write_input" => Ok(ErrorCode::InvalidWriteInput),
                    "cannot_allow_duplicate_tuples_in_one_request" => Ok(ErrorCode::CannotAllowDuplicateTuplesInOneRequest),
                    "cannot_allow_duplicate_types_in_one_request" => Ok(ErrorCode::CannotAllowDuplicateTypesInOneRequest),
                    "cannot_allow_multiple_references_to_one_relation" => Ok(ErrorCode::CannotAllowMultipleReferencesToOneRelation),
                    "invalid_continuation_token" => Ok(ErrorCode::InvalidContinuationToken),
                    "invalid_tuple_set" => Ok(ErrorCode::InvalidTupleSet),
                    "invalid_check_input" => Ok(ErrorCode::InvalidCheckInput),
                    "invalid_expand_input" => Ok(ErrorCode::InvalidExpandInput),
                    "unsupported_user_set" => Ok(ErrorCode::UnsupportedUserSet),
                    "invalid_object_format" => Ok(ErrorCode::InvalidObjectFormat),
                    "write_failed_due_to_invalid_input" => Ok(ErrorCode::WriteFailedDueToInvalidInput),
                    "authorization_model_assertions_not_found" => Ok(ErrorCode::AuthorizationModelAssertionsNotFound),
                    "latest_authorization_model_not_found" => Ok(ErrorCode::LatestAuthorizationModelNotFound),
                    "type_not_found" => Ok(ErrorCode::TypeNotFound),
                    "relation_not_found" => Ok(ErrorCode::RelationNotFound),
                    "empty_relation_definition" => Ok(ErrorCode::EmptyRelationDefinition),
                    "invalid_user" => Ok(ErrorCode::InvalidUser),
                    "invalid_tuple" => Ok(ErrorCode::InvalidTuple),
                    "unknown_relation" => Ok(ErrorCode::UnknownRelation),
                    "store_id_invalid_length" => Ok(ErrorCode::StoreIdInvalidLength),
                    "assertions_too_many_items" => Ok(ErrorCode::AssertionsTooManyItems),
                    "id_too_long" => Ok(ErrorCode::IdTooLong),
                    "authorization_model_id_too_long" => Ok(ErrorCode::AuthorizationModelIdTooLong),
                    "tuple_key_value_not_specified" => Ok(ErrorCode::TupleKeyValueNotSpecified),
                    "tuple_keys_too_many_or_too_few_items" => Ok(ErrorCode::TupleKeysTooManyOrTooFewItems),
                    "page_size_invalid" => Ok(ErrorCode::PageSizeInvalid),
                    "param_missing_value" => Ok(ErrorCode::ParamMissingValue),
                    "difference_base_missing_value" => Ok(ErrorCode::DifferenceBaseMissingValue),
                    "subtract_base_missing_value" => Ok(ErrorCode::SubtractBaseMissingValue),
                    "object_too_long" => Ok(ErrorCode::ObjectTooLong),
                    "relation_too_long" => Ok(ErrorCode::RelationTooLong),
                    "type_definitions_too_few_items" => Ok(ErrorCode::TypeDefinitionsTooFewItems),
                    "type_invalid_length" => Ok(ErrorCode::TypeInvalidLength),
                    "type_invalid_pattern" => Ok(ErrorCode::TypeInvalidPattern),
                    "relations_too_few_items" => Ok(ErrorCode::RelationsTooFewItems),
                    "relations_too_long" => Ok(ErrorCode::RelationsTooLong),
                    "relations_invalid_pattern" => Ok(ErrorCode::RelationsInvalidPattern),
                    "object_invalid_pattern" => Ok(ErrorCode::ObjectInvalidPattern),
                    "query_string_type_continuation_token_mismatch" => Ok(ErrorCode::QueryStringTypeContinuationTokenMismatch),
                    "exceeded_entity_limit" => Ok(ErrorCode::ExceededEntityLimit),
                    "invalid_contextual_tuple" => Ok(ErrorCode::InvalidContextualTuple),
                    "duplicate_contextual_tuple" => Ok(ErrorCode::DuplicateContextualTuple),
                    "invalid_authorization_model" => Ok(ErrorCode::InvalidAuthorizationModel),
                    "unsupported_schema_version" => Ok(ErrorCode::UnsupportedSchemaVersion),
                    "cancelled" => Ok(ErrorCode::Cancelled),
                    "invalid_start_time" => Ok(ErrorCode::InvalidStartTime),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorMessageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("openfga.v1.ErrorMessageRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ErrorMessageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorMessageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ErrorMessageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ErrorMessageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ErrorMessageRequest {
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ErrorMessageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpandRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if self.tuple_key.is_some() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if self.consistency != 0 {
            len += 1;
        }
        if self.contextual_tuples.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ExpandRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if let Some(v) = self.tuple_key.as_ref() {
            struct_ser.serialize_field("tuple_key", v)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if self.consistency != 0 {
            let v = ConsistencyPreference::try_from(self.consistency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.consistency)))?;
            struct_ser.serialize_field("consistency", &v)?;
        }
        if let Some(v) = self.contextual_tuples.as_ref() {
            struct_ser.serialize_field("contextual_tuples", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpandRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "tuple_key",
            "authorization_model_id",
            "consistency",
            "contextual_tuples",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            TupleKey,
            AuthorizationModelId,
            Consistency,
            ContextualTuples,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "tuple_key" => Ok(GeneratedField::TupleKey),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "consistency" => Ok(GeneratedField::Consistency),
                            "contextual_tuples" => Ok(GeneratedField::ContextualTuples),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpandRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ExpandRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpandRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut tuple_key__ = None;
                let mut authorization_model_id__ = None;
                let mut consistency__ = None;
                let mut contextual_tuples__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TupleKey => {
                            if tuple_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_key"));
                            }
                            tuple_key__ = map_.next_value()?;
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = Some(map_.next_value::<ConsistencyPreference>()? as i32);
                        }
                        GeneratedField::ContextualTuples => {
                            if contextual_tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextual_tuples"));
                            }
                            contextual_tuples__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExpandRequest {
                    store_id: store_id__.unwrap_or_default(),
                    tuple_key: tuple_key__,
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    consistency: consistency__.unwrap_or_default(),
                    contextual_tuples: contextual_tuples__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ExpandRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpandRequestTupleKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.object.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ExpandRequestTupleKey", len)?;
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpandRequestTupleKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relation",
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relation,
            Object,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "relation" => Ok(GeneratedField::Relation),
                            "object" => Ok(GeneratedField::Object),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpandRequestTupleKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ExpandRequestTupleKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpandRequestTupleKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relation__ = None;
                let mut object__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExpandRequestTupleKey {
                    relation: relation__.unwrap_or_default(),
                    object: object__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ExpandRequestTupleKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExpandResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tree.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ExpandResponse", len)?;
        if let Some(v) = self.tree.as_ref() {
            struct_ser.serialize_field("tree", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExpandResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tree",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tree,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tree" => Ok(GeneratedField::Tree),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExpandResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ExpandResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExpandResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tree__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tree => {
                            if tree__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tree"));
                            }
                            tree__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExpandResponse {
                    tree: tree__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ExpandResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ForbiddenResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ForbiddenResponse", len)?;
        if self.code != 0 {
            let v = AuthErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ForbiddenResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ForbiddenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ForbiddenResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ForbiddenResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<AuthErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ForbiddenResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ForbiddenResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStoreRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.GetStoreRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStoreRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStoreRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.GetStoreRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStoreRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetStoreRequest {
                    store_id: store_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.GetStoreRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetStoreResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if self.deleted_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.GetStoreResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("created_at", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updated_at", v)?;
        }
        if let Some(v) = self.deleted_at.as_ref() {
            struct_ser.serialize_field("deleted_at", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetStoreResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "created_at",
            "updated_at",
            "deleted_at",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            CreatedAt,
            UpdatedAt,
            DeletedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "created_at" => Ok(GeneratedField::CreatedAt),
                            "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "deleted_at" => Ok(GeneratedField::DeletedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetStoreResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.GetStoreResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetStoreResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut deleted_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated_at"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::DeletedAt => {
                            if deleted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted_at"));
                            }
                            deleted_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetStoreResponse {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    deleted_at: deleted_at__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.GetStoreResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InternalErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoInternalError => "no_internal_error",
            Self::InternalError => "internal_error",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::AlreadyExists => "already_exists",
            Self::ResourceExhausted => "resource_exhausted",
            Self::FailedPrecondition => "failed_precondition",
            Self::Aborted => "aborted",
            Self::OutOfRange => "out_of_range",
            Self::Unavailable => "unavailable",
            Self::DataLoss => "data_loss",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for InternalErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "no_internal_error",
            "internal_error",
            "deadline_exceeded",
            "already_exists",
            "resource_exhausted",
            "failed_precondition",
            "aborted",
            "out_of_range",
            "unavailable",
            "data_loss",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InternalErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "no_internal_error" => Ok(InternalErrorCode::NoInternalError),
                    "internal_error" => Ok(InternalErrorCode::InternalError),
                    "deadline_exceeded" => Ok(InternalErrorCode::DeadlineExceeded),
                    "already_exists" => Ok(InternalErrorCode::AlreadyExists),
                    "resource_exhausted" => Ok(InternalErrorCode::ResourceExhausted),
                    "failed_precondition" => Ok(InternalErrorCode::FailedPrecondition),
                    "aborted" => Ok(InternalErrorCode::Aborted),
                    "out_of_range" => Ok(InternalErrorCode::OutOfRange),
                    "unavailable" => Ok(InternalErrorCode::Unavailable),
                    "data_loss" => Ok(InternalErrorCode::DataLoss),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for InternalErrorMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.InternalErrorMessageResponse", len)?;
        if self.code != 0 {
            let v = InternalErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InternalErrorMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InternalErrorMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.InternalErrorMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InternalErrorMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<InternalErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InternalErrorMessageResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.InternalErrorMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListObjectsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        if self.contextual_tuples.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.consistency != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ListObjectsRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if let Some(v) = self.contextual_tuples.as_ref() {
            struct_ser.serialize_field("contextual_tuples", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if self.consistency != 0 {
            let v = ConsistencyPreference::try_from(self.consistency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.consistency)))?;
            struct_ser.serialize_field("consistency", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListObjectsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "authorization_model_id",
            "type",
            "relation",
            "user",
            "contextual_tuples",
            "context",
            "consistency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            AuthorizationModelId,
            Type,
            Relation,
            User,
            ContextualTuples,
            Context,
            Consistency,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "type" => Ok(GeneratedField::Type),
                            "relation" => Ok(GeneratedField::Relation),
                            "user" => Ok(GeneratedField::User),
                            "contextual_tuples" => Ok(GeneratedField::ContextualTuples),
                            "context" => Ok(GeneratedField::Context),
                            "consistency" => Ok(GeneratedField::Consistency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListObjectsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ListObjectsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListObjectsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut authorization_model_id__ = None;
                let mut r#type__ = None;
                let mut relation__ = None;
                let mut user__ = None;
                let mut contextual_tuples__ = None;
                let mut context__ = None;
                let mut consistency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextualTuples => {
                            if contextual_tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextual_tuples"));
                            }
                            contextual_tuples__ = map_.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = Some(map_.next_value::<ConsistencyPreference>()? as i32);
                        }
                    }
                }
                Ok(ListObjectsRequest {
                    store_id: store_id__.unwrap_or_default(),
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                    contextual_tuples: contextual_tuples__,
                    context: context__,
                    consistency: consistency__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ListObjectsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListObjectsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.objects.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ListObjectsResponse", len)?;
        if !self.objects.is_empty() {
            struct_ser.serialize_field("objects", &self.objects)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListObjectsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "objects",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Objects,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "objects" => Ok(GeneratedField::Objects),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListObjectsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ListObjectsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListObjectsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut objects__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Objects => {
                            if objects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objects"));
                            }
                            objects__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListObjectsResponse {
                    objects: objects__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ListObjectsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStoresRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page_size.is_some() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ListStoresRequest", len)?;
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("page_size", v)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStoresRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page_size",
            "continuation_token",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PageSize,
            ContinuationToken,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "page_size" => Ok(GeneratedField::PageSize),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListStoresRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ListStoresRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStoresRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page_size__ = None;
                let mut continuation_token__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page_size"));
                            }
                            page_size__ = map_.next_value()?;
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListStoresRequest {
                    page_size: page_size__,
                    continuation_token: continuation_token__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ListStoresRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListStoresResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stores.is_empty() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ListStoresResponse", len)?;
        if !self.stores.is_empty() {
            struct_ser.serialize_field("stores", &self.stores)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListStoresResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stores",
            "continuation_token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stores,
            ContinuationToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stores" => Ok(GeneratedField::Stores),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListStoresResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ListStoresResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListStoresResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stores__ = None;
                let mut continuation_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stores => {
                            if stores__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stores"));
                            }
                            stores__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListStoresResponse {
                    stores: stores__.unwrap_or_default(),
                    continuation_token: continuation_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ListStoresResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if self.object.is_some() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.user_filters.is_empty() {
            len += 1;
        }
        if !self.contextual_tuples.is_empty() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.consistency != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ListUsersRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.user_filters.is_empty() {
            struct_ser.serialize_field("user_filters", &self.user_filters)?;
        }
        if !self.contextual_tuples.is_empty() {
            struct_ser.serialize_field("contextual_tuples", &self.contextual_tuples)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if self.consistency != 0 {
            let v = ConsistencyPreference::try_from(self.consistency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.consistency)))?;
            struct_ser.serialize_field("consistency", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "authorization_model_id",
            "object",
            "relation",
            "user_filters",
            "contextual_tuples",
            "context",
            "consistency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            AuthorizationModelId,
            Object,
            Relation,
            UserFilters,
            ContextualTuples,
            Context,
            Consistency,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "object" => Ok(GeneratedField::Object),
                            "relation" => Ok(GeneratedField::Relation),
                            "user_filters" => Ok(GeneratedField::UserFilters),
                            "contextual_tuples" => Ok(GeneratedField::ContextualTuples),
                            "context" => Ok(GeneratedField::Context),
                            "consistency" => Ok(GeneratedField::Consistency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUsersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ListUsersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut authorization_model_id__ = None;
                let mut object__ = None;
                let mut relation__ = None;
                let mut user_filters__ = None;
                let mut contextual_tuples__ = None;
                let mut context__ = None;
                let mut consistency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map_.next_value()?;
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserFilters => {
                            if user_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user_filters"));
                            }
                            user_filters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextualTuples => {
                            if contextual_tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextual_tuples"));
                            }
                            contextual_tuples__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = Some(map_.next_value::<ConsistencyPreference>()? as i32);
                        }
                    }
                }
                Ok(ListUsersRequest {
                    store_id: store_id__.unwrap_or_default(),
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    object: object__,
                    relation: relation__.unwrap_or_default(),
                    user_filters: user_filters__.unwrap_or_default(),
                    contextual_tuples: contextual_tuples__.unwrap_or_default(),
                    context: context__,
                    consistency: consistency__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ListUsersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUsersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.users.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ListUsersResponse", len)?;
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUsersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "users",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Users,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "users" => Ok(GeneratedField::Users),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUsersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ListUsersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUsersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut users__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Users => {
                            if users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            users__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListUsersResponse {
                    users: users__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ListUsersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.relations.is_empty() {
            len += 1;
        }
        if !self.module.is_empty() {
            len += 1;
        }
        if self.source_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Metadata", len)?;
        if !self.relations.is_empty() {
            struct_ser.serialize_field("relations", &self.relations)?;
        }
        if !self.module.is_empty() {
            struct_ser.serialize_field("module", &self.module)?;
        }
        if let Some(v) = self.source_info.as_ref() {
            struct_ser.serialize_field("source_info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relations",
            "module",
            "source_info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Relations,
            Module,
            SourceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "relations" => Ok(GeneratedField::Relations),
                            "module" => Ok(GeneratedField::Module),
                            "source_info" => Ok(GeneratedField::SourceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Metadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Metadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relations__ = None;
                let mut module__ = None;
                let mut source_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Relations => {
                            if relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            relations__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Module => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module"));
                            }
                            module__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceInfo => {
                            if source_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source_info"));
                            }
                            source_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Metadata {
                    relations: relations__.unwrap_or_default(),
                    module: module__.unwrap_or_default(),
                    source_info: source_info__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Metadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotFoundErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoNotFoundError => "no_not_found_error",
            Self::UndefinedEndpoint => "undefined_endpoint",
            Self::StoreIdNotFound => "store_id_not_found",
            Self::Unimplemented => "unimplemented",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NotFoundErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "no_not_found_error",
            "undefined_endpoint",
            "store_id_not_found",
            "unimplemented",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NotFoundErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "no_not_found_error" => Ok(NotFoundErrorCode::NoNotFoundError),
                    "undefined_endpoint" => Ok(NotFoundErrorCode::UndefinedEndpoint),
                    "store_id_not_found" => Ok(NotFoundErrorCode::StoreIdNotFound),
                    "unimplemented" => Ok(NotFoundErrorCode::Unimplemented),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Object {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Object", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Object {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Object;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Object")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Object, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Object {
                    r#type: r#type__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Object", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectRelation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ObjectRelation", len)?;
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectRelation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object",
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Object,
            Relation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "object" => Ok(GeneratedField::Object),
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectRelation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ObjectRelation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ObjectRelation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object__ = None;
                let mut relation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ObjectRelation {
                    object: object__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ObjectRelation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PathUnknownErrorMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.PathUnknownErrorMessageResponse", len)?;
        if self.code != 0 {
            let v = NotFoundErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PathUnknownErrorMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PathUnknownErrorMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.PathUnknownErrorMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PathUnknownErrorMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<NotFoundErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PathUnknownErrorMessageResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.PathUnknownErrorMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadAssertionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadAssertionsRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadAssertionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "authorization_model_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            AuthorizationModelId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadAssertionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadAssertionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadAssertionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut authorization_model_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadAssertionsRequest {
                    store_id: store_id__.unwrap_or_default(),
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadAssertionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadAssertionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if !self.assertions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadAssertionsResponse", len)?;
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if !self.assertions.is_empty() {
            struct_ser.serialize_field("assertions", &self.assertions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadAssertionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_model_id",
            "assertions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationModelId,
            Assertions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "assertions" => Ok(GeneratedField::Assertions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadAssertionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadAssertionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadAssertionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization_model_id__ = None;
                let mut assertions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Assertions => {
                            if assertions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertions"));
                            }
                            assertions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadAssertionsResponse {
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    assertions: assertions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadAssertionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadAuthorizationModelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadAuthorizationModelRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadAuthorizationModelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadAuthorizationModelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadAuthorizationModelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadAuthorizationModelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadAuthorizationModelRequest {
                    store_id: store_id__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadAuthorizationModelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadAuthorizationModelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.authorization_model.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadAuthorizationModelResponse", len)?;
        if let Some(v) = self.authorization_model.as_ref() {
            struct_ser.serialize_field("authorization_model", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadAuthorizationModelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_model",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationModel,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorization_model" => Ok(GeneratedField::AuthorizationModel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadAuthorizationModelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadAuthorizationModelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadAuthorizationModelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization_model__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationModel => {
                            if authorization_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model"));
                            }
                            authorization_model__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadAuthorizationModelResponse {
                    authorization_model: authorization_model__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadAuthorizationModelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadAuthorizationModelsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if self.page_size.is_some() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadAuthorizationModelsRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("page_size", v)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadAuthorizationModelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "page_size",
            "continuation_token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            PageSize,
            ContinuationToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "page_size" => Ok(GeneratedField::PageSize),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadAuthorizationModelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadAuthorizationModelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadAuthorizationModelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut page_size__ = None;
                let mut continuation_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page_size"));
                            }
                            page_size__ = map_.next_value()?;
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadAuthorizationModelsRequest {
                    store_id: store_id__.unwrap_or_default(),
                    page_size: page_size__,
                    continuation_token: continuation_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadAuthorizationModelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadAuthorizationModelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authorization_models.is_empty() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadAuthorizationModelsResponse", len)?;
        if !self.authorization_models.is_empty() {
            struct_ser.serialize_field("authorization_models", &self.authorization_models)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadAuthorizationModelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_models",
            "continuation_token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationModels,
            ContinuationToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorization_models" => Ok(GeneratedField::AuthorizationModels),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadAuthorizationModelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadAuthorizationModelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadAuthorizationModelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization_models__ = None;
                let mut continuation_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationModels => {
                            if authorization_models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_models"));
                            }
                            authorization_models__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadAuthorizationModelsResponse {
                    authorization_models: authorization_models__.unwrap_or_default(),
                    continuation_token: continuation_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadAuthorizationModelsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadChangesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if self.page_size.is_some() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadChangesRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("page_size", v)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("start_time", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadChangesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "type",
            "page_size",
            "continuation_token",
            "start_time",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            Type,
            PageSize,
            ContinuationToken,
            StartTime,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "type" => Ok(GeneratedField::Type),
                            "page_size" => Ok(GeneratedField::PageSize),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            "start_time" => Ok(GeneratedField::StartTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadChangesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadChangesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadChangesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut r#type__ = None;
                let mut page_size__ = None;
                let mut continuation_token__ = None;
                let mut start_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page_size"));
                            }
                            page_size__ = map_.next_value()?;
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start_time"));
                            }
                            start_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadChangesRequest {
                    store_id: store_id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    page_size: page_size__,
                    continuation_token: continuation_token__.unwrap_or_default(),
                    start_time: start_time__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadChangesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadChangesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.changes.is_empty() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadChangesResponse", len)?;
        if !self.changes.is_empty() {
            struct_ser.serialize_field("changes", &self.changes)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadChangesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "changes",
            "continuation_token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Changes,
            ContinuationToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "changes" => Ok(GeneratedField::Changes),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadChangesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadChangesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadChangesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut changes__ = None;
                let mut continuation_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Changes => {
                            if changes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("changes"));
                            }
                            changes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadChangesResponse {
                    changes: changes__.unwrap_or_default(),
                    continuation_token: continuation_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadChangesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if self.tuple_key.is_some() {
            len += 1;
        }
        if self.page_size.is_some() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        if self.consistency != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if let Some(v) = self.tuple_key.as_ref() {
            struct_ser.serialize_field("tuple_key", v)?;
        }
        if let Some(v) = self.page_size.as_ref() {
            struct_ser.serialize_field("page_size", v)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        if self.consistency != 0 {
            let v = ConsistencyPreference::try_from(self.consistency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.consistency)))?;
            struct_ser.serialize_field("consistency", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "tuple_key",
            "page_size",
            "continuation_token",
            "consistency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            TupleKey,
            PageSize,
            ContinuationToken,
            Consistency,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "tuple_key" => Ok(GeneratedField::TupleKey),
                            "page_size" => Ok(GeneratedField::PageSize),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            "consistency" => Ok(GeneratedField::Consistency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut tuple_key__ = None;
                let mut page_size__ = None;
                let mut continuation_token__ = None;
                let mut consistency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TupleKey => {
                            if tuple_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_key"));
                            }
                            tuple_key__ = map_.next_value()?;
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page_size"));
                            }
                            page_size__ = map_.next_value()?;
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = Some(map_.next_value::<ConsistencyPreference>()? as i32);
                        }
                    }
                }
                Ok(ReadRequest {
                    store_id: store_id__.unwrap_or_default(),
                    tuple_key: tuple_key__,
                    page_size: page_size__,
                    continuation_token: continuation_token__.unwrap_or_default(),
                    consistency: consistency__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadRequestTupleKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.object.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadRequestTupleKey", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadRequestTupleKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "relation",
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Relation,
            Object,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "relation" => Ok(GeneratedField::Relation),
                            "object" => Ok(GeneratedField::Object),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadRequestTupleKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadRequestTupleKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadRequestTupleKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadRequestTupleKey {
                    user: user__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    object: object__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadRequestTupleKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tuples.is_empty() {
            len += 1;
        }
        if !self.continuation_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ReadResponse", len)?;
        if !self.tuples.is_empty() {
            struct_ser.serialize_field("tuples", &self.tuples)?;
        }
        if !self.continuation_token.is_empty() {
            struct_ser.serialize_field("continuation_token", &self.continuation_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuples",
            "continuation_token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tuples,
            ContinuationToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuples" => Ok(GeneratedField::Tuples),
                            "continuation_token" => Ok(GeneratedField::ContinuationToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ReadResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuples__ = None;
                let mut continuation_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tuples => {
                            if tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuples"));
                            }
                            tuples__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContinuationToken => {
                            if continuation_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continuation_token"));
                            }
                            continuation_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadResponse {
                    tuples: tuples__.unwrap_or_default(),
                    continuation_token: continuation_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ReadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Relation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.rewrite.is_some() {
            len += 1;
        }
        if self.type_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Relation", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.rewrite.as_ref() {
            struct_ser.serialize_field("rewrite", v)?;
        }
        if let Some(v) = self.type_info.as_ref() {
            struct_ser.serialize_field("typeInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "rewrite",
            "type_info",
            "typeInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Rewrite,
            TypeInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "rewrite" => Ok(GeneratedField::Rewrite),
                            "typeInfo" | "type_info" => Ok(GeneratedField::TypeInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Relation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Relation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut rewrite__ = None;
                let mut type_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rewrite => {
                            if rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewrite"));
                            }
                            rewrite__ = map_.next_value()?;
                        }
                        GeneratedField::TypeInfo => {
                            if type_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeInfo"));
                            }
                            type_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Relation {
                    name: name__.unwrap_or_default(),
                    rewrite: rewrite__,
                    type_info: type_info__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Relation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.directly_related_user_types.is_empty() {
            len += 1;
        }
        if !self.module.is_empty() {
            len += 1;
        }
        if self.source_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.RelationMetadata", len)?;
        if !self.directly_related_user_types.is_empty() {
            struct_ser.serialize_field("directly_related_user_types", &self.directly_related_user_types)?;
        }
        if !self.module.is_empty() {
            struct_ser.serialize_field("module", &self.module)?;
        }
        if let Some(v) = self.source_info.as_ref() {
            struct_ser.serialize_field("source_info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directly_related_user_types",
            "module",
            "source_info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectlyRelatedUserTypes,
            Module,
            SourceInfo,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "directly_related_user_types" => Ok(GeneratedField::DirectlyRelatedUserTypes),
                            "module" => Ok(GeneratedField::Module),
                            "source_info" => Ok(GeneratedField::SourceInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.RelationMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RelationMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directly_related_user_types__ = None;
                let mut module__ = None;
                let mut source_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DirectlyRelatedUserTypes => {
                            if directly_related_user_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directly_related_user_types"));
                            }
                            directly_related_user_types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Module => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module"));
                            }
                            module__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceInfo => {
                            if source_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source_info"));
                            }
                            source_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RelationMetadata {
                    directly_related_user_types: directly_related_user_types__.unwrap_or_default(),
                    module: module__.unwrap_or_default(),
                    source_info: source_info__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.RelationMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationReference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.condition.is_empty() {
            len += 1;
        }
        if self.relation_or_wildcard.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.RelationReference", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.condition.is_empty() {
            struct_ser.serialize_field("condition", &self.condition)?;
        }
        if let Some(v) = self.relation_or_wildcard.as_ref() {
            match v {
                relation_reference::RelationOrWildcard::Relation(v) => {
                    struct_ser.serialize_field("relation", v)?;
                }
                relation_reference::RelationOrWildcard::Wildcard(v) => {
                    struct_ser.serialize_field("wildcard", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "condition",
            "relation",
            "wildcard",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Condition,
            Relation,
            Wildcard,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "condition" => Ok(GeneratedField::Condition),
                            "relation" => Ok(GeneratedField::Relation),
                            "wildcard" => Ok(GeneratedField::Wildcard),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.RelationReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RelationReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut condition__ = None;
                let mut relation_or_wildcard__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation_or_wildcard__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation_or_wildcard__ = map_.next_value::<::std::option::Option<_>>()?.map(relation_reference::RelationOrWildcard::Relation);
                        }
                        GeneratedField::Wildcard => {
                            if relation_or_wildcard__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wildcard"));
                            }
                            relation_or_wildcard__ = map_.next_value::<::std::option::Option<_>>()?.map(relation_reference::RelationOrWildcard::Wildcard)
;
                        }
                    }
                }
                Ok(RelationReference {
                    r#type: r#type__.unwrap_or_default(),
                    condition: condition__.unwrap_or_default(),
                    relation_or_wildcard: relation_or_wildcard__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.RelationReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationTypeInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.directly_related_user_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.RelationTypeInfo", len)?;
        if !self.directly_related_user_types.is_empty() {
            struct_ser.serialize_field("directly_related_user_types", &self.directly_related_user_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationTypeInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "directly_related_user_types",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DirectlyRelatedUserTypes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "directly_related_user_types" => Ok(GeneratedField::DirectlyRelatedUserTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationTypeInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.RelationTypeInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RelationTypeInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut directly_related_user_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DirectlyRelatedUserTypes => {
                            if directly_related_user_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directly_related_user_types"));
                            }
                            directly_related_user_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RelationTypeInfo {
                    directly_related_user_types: directly_related_user_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.RelationTypeInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationshipCondition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.RelationshipCondition", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationshipCondition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "context",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Context,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "context" => Ok(GeneratedField::Context),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationshipCondition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.RelationshipCondition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RelationshipCondition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut context__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RelationshipCondition {
                    name: name__.unwrap_or_default(),
                    context: context__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.RelationshipCondition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.SourceInfo", len)?;
        if !self.file.is_empty() {
            struct_ser.serialize_field("file", &self.file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SourceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            File,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "file" => Ok(GeneratedField::File),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.SourceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SourceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::File => {
                            if file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("file"));
                            }
                            file__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SourceInfo {
                    file: file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.SourceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Store {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if self.deleted_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Store", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("created_at", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updated_at", v)?;
        }
        if let Some(v) = self.deleted_at.as_ref() {
            struct_ser.serialize_field("deleted_at", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Store {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "created_at",
            "updated_at",
            "deleted_at",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            CreatedAt,
            UpdatedAt,
            DeletedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "created_at" => Ok(GeneratedField::CreatedAt),
                            "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "deleted_at" => Ok(GeneratedField::DeletedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Store;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Store")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Store, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut deleted_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated_at"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                        GeneratedField::DeletedAt => {
                            if deleted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deleted_at"));
                            }
                            deleted_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Store {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    deleted_at: deleted_at__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Store", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamedListObjectsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.user.is_empty() {
            len += 1;
        }
        if self.contextual_tuples.is_some() {
            len += 1;
        }
        if self.context.is_some() {
            len += 1;
        }
        if self.consistency != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.StreamedListObjectsRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if let Some(v) = self.contextual_tuples.as_ref() {
            struct_ser.serialize_field("contextual_tuples", v)?;
        }
        if let Some(v) = self.context.as_ref() {
            struct_ser.serialize_field("context", v)?;
        }
        if self.consistency != 0 {
            let v = ConsistencyPreference::try_from(self.consistency)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.consistency)))?;
            struct_ser.serialize_field("consistency", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamedListObjectsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "authorization_model_id",
            "type",
            "relation",
            "user",
            "contextual_tuples",
            "context",
            "consistency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            AuthorizationModelId,
            Type,
            Relation,
            User,
            ContextualTuples,
            Context,
            Consistency,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "type" => Ok(GeneratedField::Type),
                            "relation" => Ok(GeneratedField::Relation),
                            "user" => Ok(GeneratedField::User),
                            "contextual_tuples" => Ok(GeneratedField::ContextualTuples),
                            "context" => Ok(GeneratedField::Context),
                            "consistency" => Ok(GeneratedField::Consistency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamedListObjectsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.StreamedListObjectsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamedListObjectsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut authorization_model_id__ = None;
                let mut r#type__ = None;
                let mut relation__ = None;
                let mut user__ = None;
                let mut contextual_tuples__ = None;
                let mut context__ = None;
                let mut consistency__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContextualTuples => {
                            if contextual_tuples__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextual_tuples"));
                            }
                            contextual_tuples__ = map_.next_value()?;
                        }
                        GeneratedField::Context => {
                            if context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            context__ = map_.next_value()?;
                        }
                        GeneratedField::Consistency => {
                            if consistency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistency"));
                            }
                            consistency__ = Some(map_.next_value::<ConsistencyPreference>()? as i32);
                        }
                    }
                }
                Ok(StreamedListObjectsRequest {
                    store_id: store_id__.unwrap_or_default(),
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    user: user__.unwrap_or_default(),
                    contextual_tuples: contextual_tuples__,
                    context: context__,
                    consistency: consistency__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.StreamedListObjectsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamedListObjectsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.StreamedListObjectsResponse", len)?;
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamedListObjectsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Object,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "object" => Ok(GeneratedField::Object),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamedListObjectsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.StreamedListObjectsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamedListObjectsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StreamedListObjectsResponse {
                    object: object__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.StreamedListObjectsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tuple {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Tuple", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tuple {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tuple;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Tuple")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Tuple, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map_.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Tuple {
                    key: key__,
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Tuple", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TupleChange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tuple_key.is_some() {
            len += 1;
        }
        if self.operation != 0 {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.TupleChange", len)?;
        if let Some(v) = self.tuple_key.as_ref() {
            struct_ser.serialize_field("tuple_key", v)?;
        }
        if self.operation != 0 {
            let v = TupleOperation::try_from(self.operation)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.operation)))?;
            struct_ser.serialize_field("operation", &v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TupleChange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuple_key",
            "operation",
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TupleKey,
            Operation,
            Timestamp,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuple_key" => Ok(GeneratedField::TupleKey),
                            "operation" => Ok(GeneratedField::Operation),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TupleChange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.TupleChange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TupleChange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuple_key__ = None;
                let mut operation__ = None;
                let mut timestamp__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TupleKey => {
                            if tuple_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_key"));
                            }
                            tuple_key__ = map_.next_value()?;
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map_.next_value::<TupleOperation>()? as i32);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TupleChange {
                    tuple_key: tuple_key__,
                    operation: operation__.unwrap_or_default(),
                    timestamp: timestamp__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.TupleChange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TupleKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.object.is_empty() {
            len += 1;
        }
        if self.condition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.TupleKey", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        if let Some(v) = self.condition.as_ref() {
            struct_ser.serialize_field("condition", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TupleKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "relation",
            "object",
            "condition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Relation,
            Object,
            Condition,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "relation" => Ok(GeneratedField::Relation),
                            "object" => Ok(GeneratedField::Object),
                            "condition" => Ok(GeneratedField::Condition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TupleKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.TupleKey")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TupleKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                let mut condition__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Condition => {
                            if condition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            condition__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TupleKey {
                    user: user__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    object: object__.unwrap_or_default(),
                    condition: condition__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.TupleKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TupleKeyWithoutCondition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.object.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.TupleKeyWithoutCondition", len)?;
        if !self.user.is_empty() {
            struct_ser.serialize_field("user", &self.user)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.object.is_empty() {
            struct_ser.serialize_field("object", &self.object)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TupleKeyWithoutCondition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user",
            "relation",
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            User,
            Relation,
            Object,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "user" => Ok(GeneratedField::User),
                            "relation" => Ok(GeneratedField::Relation),
                            "object" => Ok(GeneratedField::Object),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TupleKeyWithoutCondition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.TupleKeyWithoutCondition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TupleKeyWithoutCondition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TupleKeyWithoutCondition {
                    user: user__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    object: object__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.TupleKeyWithoutCondition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TupleKeys {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tuple_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.TupleKeys", len)?;
        if !self.tuple_keys.is_empty() {
            struct_ser.serialize_field("tuple_keys", &self.tuple_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TupleKeys {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuple_keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TupleKeys,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuple_keys" => Ok(GeneratedField::TupleKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TupleKeys;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.TupleKeys")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TupleKeys, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuple_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TupleKeys => {
                            if tuple_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_keys"));
                            }
                            tuple_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TupleKeys {
                    tuple_keys: tuple_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.TupleKeys", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TupleOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Write => "TUPLE_OPERATION_WRITE",
            Self::Delete => "TUPLE_OPERATION_DELETE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TupleOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TUPLE_OPERATION_WRITE",
            "TUPLE_OPERATION_DELETE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TupleOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TUPLE_OPERATION_WRITE" => Ok(TupleOperation::Write),
                    "TUPLE_OPERATION_DELETE" => Ok(TupleOperation::Delete),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TupleToUserset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tupleset.is_some() {
            len += 1;
        }
        if self.computed_userset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.TupleToUserset", len)?;
        if let Some(v) = self.tupleset.as_ref() {
            struct_ser.serialize_field("tupleset", v)?;
        }
        if let Some(v) = self.computed_userset.as_ref() {
            struct_ser.serialize_field("computedUserset", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TupleToUserset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tupleset",
            "computed_userset",
            "computedUserset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tupleset,
            ComputedUserset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tupleset" => Ok(GeneratedField::Tupleset),
                            "computedUserset" | "computed_userset" => Ok(GeneratedField::ComputedUserset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TupleToUserset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.TupleToUserset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TupleToUserset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tupleset__ = None;
                let mut computed_userset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tupleset => {
                            if tupleset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tupleset"));
                            }
                            tupleset__ = map_.next_value()?;
                        }
                        GeneratedField::ComputedUserset => {
                            if computed_userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("computedUserset"));
                            }
                            computed_userset__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TupleToUserset {
                    tupleset: tupleset__,
                    computed_userset: computed_userset__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.TupleToUserset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypeDefinition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.relations.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.TypeDefinition", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.relations.is_empty() {
            struct_ser.serialize_field("relations", &self.relations)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypeDefinition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "relations",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Relations,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "relations" => Ok(GeneratedField::Relations),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypeDefinition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.TypeDefinition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TypeDefinition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut relations__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relations => {
                            if relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            relations__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TypeDefinition {
                    r#type: r#type__.unwrap_or_default(),
                    relations: relations__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.TypeDefinition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypedWildcard {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.TypedWildcard", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypedWildcard {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TypedWildcard;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.TypedWildcard")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TypedWildcard, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TypedWildcard {
                    r#type: r#type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.TypedWildcard", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnauthenticatedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UnauthenticatedResponse", len)?;
        if self.code != 0 {
            let v = ErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnauthenticatedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnauthenticatedResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UnauthenticatedResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnauthenticatedResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<ErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnauthenticatedResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UnauthenticatedResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnprocessableContentErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoThrottledErrorCode => "no_throttled_error_code",
            Self::ThrottledTimeoutError => "throttled_timeout_error",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UnprocessableContentErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "no_throttled_error_code",
            "throttled_timeout_error",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnprocessableContentErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "no_throttled_error_code" => Ok(UnprocessableContentErrorCode::NoThrottledErrorCode),
                    "throttled_timeout_error" => Ok(UnprocessableContentErrorCode::ThrottledTimeoutError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UnprocessableContentMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UnprocessableContentMessageResponse", len)?;
        if self.code != 0 {
            let v = UnprocessableContentErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnprocessableContentMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnprocessableContentMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UnprocessableContentMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UnprocessableContentMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<UnprocessableContentErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UnprocessableContentMessageResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UnprocessableContentMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateStoreRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UpdateStoreRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateStoreRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateStoreRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UpdateStoreRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateStoreRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateStoreRequest {
                    store_id: store_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UpdateStoreRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateStoreResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UpdateStoreResponse", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("created_at", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updated_at", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateStoreResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
            "created_at",
            "updated_at",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            "created_at" => Ok(GeneratedField::CreatedAt),
                            "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateStoreResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UpdateStoreResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateStoreResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut name__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("created_at"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updated_at"));
                            }
                            updated_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateStoreResponse {
                    id: id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UpdateStoreResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.User", len)?;
        if let Some(v) = self.user.as_ref() {
            match v {
                user::User::Object(v) => {
                    struct_ser.serialize_field("object", v)?;
                }
                user::User::Userset(v) => {
                    struct_ser.serialize_field("userset", v)?;
                }
                user::User::Wildcard(v) => {
                    struct_ser.serialize_field("wildcard", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object",
            "userset",
            "wildcard",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Object,
            Userset,
            Wildcard,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "object" => Ok(GeneratedField::Object),
                            "userset" => Ok(GeneratedField::Userset),
                            "wildcard" => Ok(GeneratedField::Wildcard),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Object => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            user__ = map_.next_value::<::std::option::Option<_>>()?.map(user::User::Object)
;
                        }
                        GeneratedField::Userset => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userset"));
                            }
                            user__ = map_.next_value::<::std::option::Option<_>>()?.map(user::User::Userset)
;
                        }
                        GeneratedField::Wildcard => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wildcard"));
                            }
                            user__ = map_.next_value::<::std::option::Option<_>>()?.map(user::User::Wildcard)
;
                        }
                    }
                }
                Ok(User {
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.User", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserTypeFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UserTypeFilter", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserTypeFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Relation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserTypeFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UserTypeFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserTypeFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut relation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserTypeFilter {
                    r#type: r#type__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UserTypeFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Userset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.userset.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Userset", len)?;
        if let Some(v) = self.userset.as_ref() {
            match v {
                userset::Userset::This(v) => {
                    struct_ser.serialize_field("this", v)?;
                }
                userset::Userset::ComputedUserset(v) => {
                    struct_ser.serialize_field("computedUserset", v)?;
                }
                userset::Userset::TupleToUserset(v) => {
                    struct_ser.serialize_field("tupleToUserset", v)?;
                }
                userset::Userset::Union(v) => {
                    struct_ser.serialize_field("union", v)?;
                }
                userset::Userset::Intersection(v) => {
                    struct_ser.serialize_field("intersection", v)?;
                }
                userset::Userset::Difference(v) => {
                    struct_ser.serialize_field("difference", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Userset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "this",
            "computed_userset",
            "computedUserset",
            "tuple_to_userset",
            "tupleToUserset",
            "union",
            "intersection",
            "difference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            This,
            ComputedUserset,
            TupleToUserset,
            Union,
            Intersection,
            Difference,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "this" => Ok(GeneratedField::This),
                            "computedUserset" | "computed_userset" => Ok(GeneratedField::ComputedUserset),
                            "tupleToUserset" | "tuple_to_userset" => Ok(GeneratedField::TupleToUserset),
                            "union" => Ok(GeneratedField::Union),
                            "intersection" => Ok(GeneratedField::Intersection),
                            "difference" => Ok(GeneratedField::Difference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Userset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Userset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Userset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut userset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::This => {
                            if userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("this"));
                            }
                            userset__ = map_.next_value::<::std::option::Option<_>>()?.map(userset::Userset::This)
;
                        }
                        GeneratedField::ComputedUserset => {
                            if userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("computedUserset"));
                            }
                            userset__ = map_.next_value::<::std::option::Option<_>>()?.map(userset::Userset::ComputedUserset)
;
                        }
                        GeneratedField::TupleToUserset => {
                            if userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tupleToUserset"));
                            }
                            userset__ = map_.next_value::<::std::option::Option<_>>()?.map(userset::Userset::TupleToUserset)
;
                        }
                        GeneratedField::Union => {
                            if userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("union"));
                            }
                            userset__ = map_.next_value::<::std::option::Option<_>>()?.map(userset::Userset::Union)
;
                        }
                        GeneratedField::Intersection => {
                            if userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intersection"));
                            }
                            userset__ = map_.next_value::<::std::option::Option<_>>()?.map(userset::Userset::Intersection)
;
                        }
                        GeneratedField::Difference => {
                            if userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("difference"));
                            }
                            userset__ = map_.next_value::<::std::option::Option<_>>()?.map(userset::Userset::Difference)
;
                        }
                    }
                }
                Ok(Userset {
                    userset: userset__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Userset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UsersetTree {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree", len)?;
        if let Some(v) = self.root.as_ref() {
            struct_ser.serialize_field("root", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsersetTree {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Root,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "root" => Ok(GeneratedField::Root),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsersetTree;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsersetTree, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Root => {
                            if root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UsersetTree {
                    root: root__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for userset_tree::Computed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.userset.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree.Computed", len)?;
        if !self.userset.is_empty() {
            struct_ser.serialize_field("userset", &self.userset)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for userset_tree::Computed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "userset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Userset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "userset" => Ok(GeneratedField::Userset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = userset_tree::Computed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree.Computed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<userset_tree::Computed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut userset__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Userset => {
                            if userset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userset"));
                            }
                            userset__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(userset_tree::Computed {
                    userset: userset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree.Computed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for userset_tree::Difference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base.is_some() {
            len += 1;
        }
        if self.subtract.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree.Difference", len)?;
        if let Some(v) = self.base.as_ref() {
            struct_ser.serialize_field("base", v)?;
        }
        if let Some(v) = self.subtract.as_ref() {
            struct_ser.serialize_field("subtract", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for userset_tree::Difference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base",
            "subtract",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Base,
            Subtract,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "base" => Ok(GeneratedField::Base),
                            "subtract" => Ok(GeneratedField::Subtract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = userset_tree::Difference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree.Difference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<userset_tree::Difference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base__ = None;
                let mut subtract__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = map_.next_value()?;
                        }
                        GeneratedField::Subtract => {
                            if subtract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subtract"));
                            }
                            subtract__ = map_.next_value()?;
                        }
                    }
                }
                Ok(userset_tree::Difference {
                    base: base__,
                    subtract: subtract__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree.Difference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for userset_tree::Leaf {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree.Leaf", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                userset_tree::leaf::Value::Users(v) => {
                    struct_ser.serialize_field("users", v)?;
                }
                userset_tree::leaf::Value::Computed(v) => {
                    struct_ser.serialize_field("computed", v)?;
                }
                userset_tree::leaf::Value::TupleToUserset(v) => {
                    struct_ser.serialize_field("tupleToUserset", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for userset_tree::Leaf {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "users",
            "computed",
            "tuple_to_userset",
            "tupleToUserset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Users,
            Computed,
            TupleToUserset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "users" => Ok(GeneratedField::Users),
                            "computed" => Ok(GeneratedField::Computed),
                            "tupleToUserset" | "tuple_to_userset" => Ok(GeneratedField::TupleToUserset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = userset_tree::Leaf;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree.Leaf")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<userset_tree::Leaf, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Users => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(userset_tree::leaf::Value::Users)
;
                        }
                        GeneratedField::Computed => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("computed"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(userset_tree::leaf::Value::Computed)
;
                        }
                        GeneratedField::TupleToUserset => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tupleToUserset"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(userset_tree::leaf::Value::TupleToUserset)
;
                        }
                    }
                }
                Ok(userset_tree::Leaf {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree.Leaf", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for userset_tree::Node {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree.Node", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                userset_tree::node::Value::Leaf(v) => {
                    struct_ser.serialize_field("leaf", v)?;
                }
                userset_tree::node::Value::Difference(v) => {
                    struct_ser.serialize_field("difference", v)?;
                }
                userset_tree::node::Value::Union(v) => {
                    struct_ser.serialize_field("union", v)?;
                }
                userset_tree::node::Value::Intersection(v) => {
                    struct_ser.serialize_field("intersection", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for userset_tree::Node {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "leaf",
            "difference",
            "union",
            "intersection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Leaf,
            Difference,
            Union,
            Intersection,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "leaf" => Ok(GeneratedField::Leaf),
                            "difference" => Ok(GeneratedField::Difference),
                            "union" => Ok(GeneratedField::Union),
                            "intersection" => Ok(GeneratedField::Intersection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = userset_tree::Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree.Node")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<userset_tree::Node, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Leaf => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leaf"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(userset_tree::node::Value::Leaf)
;
                        }
                        GeneratedField::Difference => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("difference"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(userset_tree::node::Value::Difference)
;
                        }
                        GeneratedField::Union => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("union"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(userset_tree::node::Value::Union)
;
                        }
                        GeneratedField::Intersection => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intersection"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(userset_tree::node::Value::Intersection)
;
                        }
                    }
                }
                Ok(userset_tree::Node {
                    name: name__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree.Node", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for userset_tree::Nodes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nodes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree.Nodes", len)?;
        if !self.nodes.is_empty() {
            struct_ser.serialize_field("nodes", &self.nodes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for userset_tree::Nodes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nodes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nodes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nodes" => Ok(GeneratedField::Nodes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = userset_tree::Nodes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree.Nodes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<userset_tree::Nodes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nodes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nodes => {
                            if nodes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            nodes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(userset_tree::Nodes {
                    nodes: nodes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree.Nodes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for userset_tree::TupleToUserset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tupleset.is_empty() {
            len += 1;
        }
        if !self.computed.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree.TupleToUserset", len)?;
        if !self.tupleset.is_empty() {
            struct_ser.serialize_field("tupleset", &self.tupleset)?;
        }
        if !self.computed.is_empty() {
            struct_ser.serialize_field("computed", &self.computed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for userset_tree::TupleToUserset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tupleset",
            "computed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tupleset,
            Computed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tupleset" => Ok(GeneratedField::Tupleset),
                            "computed" => Ok(GeneratedField::Computed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = userset_tree::TupleToUserset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree.TupleToUserset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<userset_tree::TupleToUserset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tupleset__ = None;
                let mut computed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tupleset => {
                            if tupleset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tupleset"));
                            }
                            tupleset__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Computed => {
                            if computed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("computed"));
                            }
                            computed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(userset_tree::TupleToUserset {
                    tupleset: tupleset__.unwrap_or_default(),
                    computed: computed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree.TupleToUserset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for userset_tree::Users {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.users.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetTree.Users", len)?;
        if !self.users.is_empty() {
            struct_ser.serialize_field("users", &self.users)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for userset_tree::Users {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "users",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Users,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "users" => Ok(GeneratedField::Users),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = userset_tree::Users;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetTree.Users")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<userset_tree::Users, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut users__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Users => {
                            if users__.is_some() {
                                return Err(serde::de::Error::duplicate_field("users"));
                            }
                            users__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(userset_tree::Users {
                    users: users__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetTree.Users", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UsersetUser {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.UsersetUser", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UsersetUser {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "id",
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Id,
            Relation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "id" => Ok(GeneratedField::Id),
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UsersetUser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.UsersetUser")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UsersetUser, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut id__ = None;
                let mut relation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UsersetUser {
                    r#type: r#type__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.UsersetUser", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Usersets {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.child.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.Usersets", len)?;
        if !self.child.is_empty() {
            struct_ser.serialize_field("child", &self.child)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Usersets {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "child",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Child,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "child" => Ok(GeneratedField::Child),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Usersets;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Usersets")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Usersets, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut child__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Child => {
                            if child__.is_some() {
                                return Err(serde::de::Error::duplicate_field("child"));
                            }
                            child__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Usersets {
                    child: child__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Usersets", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidationErrorMessageResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.ValidationErrorMessageResponse", len)?;
        if self.code != 0 {
            let v = ErrorCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidationErrorMessageResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidationErrorMessageResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.ValidationErrorMessageResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidationErrorMessageResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<ErrorCode>()? as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ValidationErrorMessageResponse {
                    code: code__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.ValidationErrorMessageResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Wildcard {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("openfga.v1.Wildcard", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Wildcard {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Wildcard;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.Wildcard")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Wildcard, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Wildcard {
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.Wildcard", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteAssertionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        if !self.assertions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.WriteAssertionsRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        if !self.assertions.is_empty() {
            struct_ser.serialize_field("assertions", &self.assertions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteAssertionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "authorization_model_id",
            "assertions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            AuthorizationModelId,
            Assertions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            "assertions" => Ok(GeneratedField::Assertions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteAssertionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteAssertionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteAssertionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut authorization_model_id__ = None;
                let mut assertions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Assertions => {
                            if assertions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assertions"));
                            }
                            assertions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WriteAssertionsRequest {
                    store_id: store_id__.unwrap_or_default(),
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                    assertions: assertions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteAssertionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteAssertionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("openfga.v1.WriteAssertionsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteAssertionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteAssertionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteAssertionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteAssertionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WriteAssertionsResponse {
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteAssertionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteAuthorizationModelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if !self.type_definitions.is_empty() {
            len += 1;
        }
        if !self.schema_version.is_empty() {
            len += 1;
        }
        if !self.conditions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.WriteAuthorizationModelRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if !self.type_definitions.is_empty() {
            struct_ser.serialize_field("type_definitions", &self.type_definitions)?;
        }
        if !self.schema_version.is_empty() {
            struct_ser.serialize_field("schema_version", &self.schema_version)?;
        }
        if !self.conditions.is_empty() {
            struct_ser.serialize_field("conditions", &self.conditions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteAuthorizationModelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "type_definitions",
            "schema_version",
            "conditions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            TypeDefinitions,
            SchemaVersion,
            Conditions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "type_definitions" => Ok(GeneratedField::TypeDefinitions),
                            "schema_version" => Ok(GeneratedField::SchemaVersion),
                            "conditions" => Ok(GeneratedField::Conditions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteAuthorizationModelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteAuthorizationModelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteAuthorizationModelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut type_definitions__ = None;
                let mut schema_version__ = None;
                let mut conditions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TypeDefinitions => {
                            if type_definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type_definitions"));
                            }
                            type_definitions__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SchemaVersion => {
                            if schema_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema_version"));
                            }
                            schema_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Conditions => {
                            if conditions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("conditions"));
                            }
                            conditions__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(WriteAuthorizationModelRequest {
                    store_id: store_id__.unwrap_or_default(),
                    type_definitions: type_definitions__.unwrap_or_default(),
                    schema_version: schema_version__.unwrap_or_default(),
                    conditions: conditions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteAuthorizationModelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteAuthorizationModelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.WriteAuthorizationModelResponse", len)?;
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteAuthorizationModelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_model_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationModelId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteAuthorizationModelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteAuthorizationModelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteAuthorizationModelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization_model_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WriteAuthorizationModelResponse {
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteAuthorizationModelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.store_id.is_empty() {
            len += 1;
        }
        if self.writes.is_some() {
            len += 1;
        }
        if self.deletes.is_some() {
            len += 1;
        }
        if !self.authorization_model_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.WriteRequest", len)?;
        if !self.store_id.is_empty() {
            struct_ser.serialize_field("store_id", &self.store_id)?;
        }
        if let Some(v) = self.writes.as_ref() {
            struct_ser.serialize_field("writes", v)?;
        }
        if let Some(v) = self.deletes.as_ref() {
            struct_ser.serialize_field("deletes", v)?;
        }
        if !self.authorization_model_id.is_empty() {
            struct_ser.serialize_field("authorization_model_id", &self.authorization_model_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "store_id",
            "writes",
            "deletes",
            "authorization_model_id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StoreId,
            Writes,
            Deletes,
            AuthorizationModelId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "store_id" => Ok(GeneratedField::StoreId),
                            "writes" => Ok(GeneratedField::Writes),
                            "deletes" => Ok(GeneratedField::Deletes),
                            "authorization_model_id" => Ok(GeneratedField::AuthorizationModelId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut store_id__ = None;
                let mut writes__ = None;
                let mut deletes__ = None;
                let mut authorization_model_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StoreId => {
                            if store_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("store_id"));
                            }
                            store_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Writes => {
                            if writes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("writes"));
                            }
                            writes__ = map_.next_value()?;
                        }
                        GeneratedField::Deletes => {
                            if deletes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletes"));
                            }
                            deletes__ = map_.next_value()?;
                        }
                        GeneratedField::AuthorizationModelId => {
                            if authorization_model_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization_model_id"));
                            }
                            authorization_model_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WriteRequest {
                    store_id: store_id__.unwrap_or_default(),
                    writes: writes__,
                    deletes: deletes__,
                    authorization_model_id: authorization_model_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteRequestDeletes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tuple_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.WriteRequestDeletes", len)?;
        if !self.tuple_keys.is_empty() {
            struct_ser.serialize_field("tuple_keys", &self.tuple_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteRequestDeletes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuple_keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TupleKeys,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuple_keys" => Ok(GeneratedField::TupleKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteRequestDeletes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteRequestDeletes")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteRequestDeletes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuple_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TupleKeys => {
                            if tuple_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_keys"));
                            }
                            tuple_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WriteRequestDeletes {
                    tuple_keys: tuple_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteRequestDeletes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteRequestWrites {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tuple_keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("openfga.v1.WriteRequestWrites", len)?;
        if !self.tuple_keys.is_empty() {
            struct_ser.serialize_field("tuple_keys", &self.tuple_keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteRequestWrites {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tuple_keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TupleKeys,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tuple_keys" => Ok(GeneratedField::TupleKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteRequestWrites;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteRequestWrites")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteRequestWrites, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tuple_keys__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TupleKeys => {
                            if tuple_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tuple_keys"));
                            }
                            tuple_keys__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WriteRequestWrites {
                    tuple_keys: tuple_keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteRequestWrites", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WriteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("openfga.v1.WriteResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WriteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WriteResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct openfga.v1.WriteResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WriteResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WriteResponse {
                })
            }
        }
        deserializer.deserialize_struct("openfga.v1.WriteResponse", FIELDS, GeneratedVisitor)
    }
}
