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
pub struct SubmitSelfServiceVerificationFlowWithLinkMethodBody {
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Email to Verify  Needs to be set when initiating the flow. If the email is a registered verification email, a verification link will be sent. If the email is not known, a email with details on what happened will be sent instead.  format: email
    #[serde(rename = "email")]
    pub email: String,
    /// Method supports `link` only right now.
    #[serde(rename = "method")]
    pub method: String,
}

impl SubmitSelfServiceVerificationFlowWithLinkMethodBody {
    pub fn new(email: String, method: String) -> SubmitSelfServiceVerificationFlowWithLinkMethodBody {
        SubmitSelfServiceVerificationFlowWithLinkMethodBody {
            csrf_token: None,
            email,
            method,
        }
    }
}


