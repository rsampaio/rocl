/*
 * Open Service Broker API
 *
 * The Open Service Broker API defines an HTTP(S) interface between Platforms and Service Brokers.
 *
 * The version of the OpenAPI document: master - might contain changes that are not yet released
 * Contact: open-service-broker-api@googlegroups.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBindingResouceObject {
    #[serde(rename = "app_guid", skip_serializing_if = "Option::is_none")]
    pub app_guid: Option<String>,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
}

impl ServiceBindingResouceObject {
    pub fn new() -> ServiceBindingResouceObject {
        ServiceBindingResouceObject {
            app_guid: None,
            route: None,
        }
    }
}


