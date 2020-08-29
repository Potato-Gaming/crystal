/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BindingAsyncCancelEvent : Represents a cancelled asynchronous operation.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindingAsyncCancelEvent {
    /// Asynchronous operation token
    #[serde(rename = "asyncToken", skip_serializing_if = "Option::is_none")]
    pub async_token: Option<i32>,
}

impl BindingAsyncCancelEvent {
    /// Represents a cancelled asynchronous operation.
    pub fn new() -> BindingAsyncCancelEvent {
        BindingAsyncCancelEvent {
            async_token: None,
        }
    }
}

