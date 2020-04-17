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
pub struct ServiceInstanceResource {
    #[serde(rename = "service_id", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "plan_id", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "dashboard_url", skip_serializing_if = "Option::is_none")]
    pub dashboard_url: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl ServiceInstanceResource {
    pub fn new() -> ServiceInstanceResource {
        ServiceInstanceResource {
            service_id: None,
            plan_id: None,
            dashboard_url: None,
            parameters: None,
        }
    }
}

