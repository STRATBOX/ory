/*
 * Ory Kratos
 *
 * Welcome to the Ory Kratos HTTP API documentation!
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AdminCreateIdentityBody {
    /// SchemaID is the ID of the JSON Schema to be used for validating the identity's traits.
    #[serde(rename = "schema_id")]
    pub schema_id: String,
    /// Traits represent an identity's traits. The identity is able to create, modify, and delete traits in a self-service manner. The input will always be validated against the JSON Schema defined in `schema_url`.
    #[serde(rename = "traits")]
    pub traits: serde_json::Value,
}

impl AdminCreateIdentityBody {
    pub fn new(schema_id: String, traits: serde_json::Value) -> AdminCreateIdentityBody {
        AdminCreateIdentityBody {
            schema_id,
            traits,
        }
    }
}


