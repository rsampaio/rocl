/*
 * Open Service Broker API
 *
 * The Open Service Broker API defines an HTTP(S) interface between Platforms and Service Brokers.
 *
 * The version of the OpenAPI document: master - might contain changes that are not yet released
 * Contact: open-service-broker-api@googlegroups.com
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBindingRequest {
    /// See [Context Conventions](https://github.com/openservicebrokerapi/servicebroker/blob/master/profile.md#context-object) for more details.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[serde(rename = "service_id")]
    pub service_id: String,
    #[serde(rename = "plan_id")]
    pub plan_id: String,
    #[serde(rename = "app_guid", skip_serializing_if = "Option::is_none")]
    pub app_guid: Option<String>,
    #[serde(rename = "bind_resource", skip_serializing_if = "Option::is_none")]
    pub bind_resource: Option<crate::models::ServiceBindingResouceObject>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl ServiceBindingRequest {
    pub fn new(service_id: String, plan_id: String) -> ServiceBindingRequest {
        ServiceBindingRequest {
            context: None,
            service_id: service_id,
            plan_id: plan_id,
            app_guid: None,
            bind_resource: None,
            parameters: None,
        }
    }
}

