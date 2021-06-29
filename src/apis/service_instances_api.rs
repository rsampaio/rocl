/*
 * Open Service Broker API
 *
 * The Open Service Broker API defines an HTTP(S) interface between Platforms and Service Brokers.
 *
 * The version of the OpenAPI document: master - might contain changes that are not yet released
 * Contact: open-service-broker-api@googlegroups.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `service_instance_deprovision`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceInstanceDeprovisionError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status410(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `service_instance_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceInstanceGetError {
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `service_instance_last_operation_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceInstanceLastOperationGetError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    Status410(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `service_instance_provision`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceInstanceProvisionError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status409(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `service_instance_update`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceInstanceUpdateError {
    Status400(crate::models::Error),
    Status401(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}


pub async fn service_instance_deprovision(configuration: &configuration::Configuration, x_broker_api_version: &str, instance_id: &str, service_id: &str, plan_id: &str, x_broker_api_originating_identity: Option<&str>, accepts_incomplete: Option<bool>) -> Result<serde_json::Value, Error<ServiceInstanceDeprovisionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/service_instances/{instance_id}", configuration.base_path, instance_id=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("service_id", &service_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("plan_id", &plan_id.to_string())]);
    if let Some(ref local_var_str) = accepts_incomplete {
        local_var_req_builder = local_var_req_builder.query(&[("accepts_incomplete", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
    if let Some(local_var_param_value) = x_broker_api_originating_identity {
        local_var_req_builder = local_var_req_builder.header("X-Broker-API-Originating-Identity", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceInstanceDeprovisionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn service_instance_get(configuration: &configuration::Configuration, x_broker_api_version: &str, instance_id: &str, x_broker_api_originating_identity: Option<&str>, service_id: Option<&str>, plan_id: Option<&str>) -> Result<crate::models::ServiceInstanceResource, Error<ServiceInstanceGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/service_instances/{instance_id}", configuration.base_path, instance_id=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = service_id {
        local_var_req_builder = local_var_req_builder.query(&[("service_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = plan_id {
        local_var_req_builder = local_var_req_builder.query(&[("plan_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
    if let Some(local_var_param_value) = x_broker_api_originating_identity {
        local_var_req_builder = local_var_req_builder.header("X-Broker-API-Originating-Identity", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceInstanceGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn service_instance_last_operation_get(configuration: &configuration::Configuration, x_broker_api_version: &str, instance_id: &str, service_id: Option<&str>, plan_id: Option<&str>, operation: Option<&str>) -> Result<crate::models::LastOperationResource, Error<ServiceInstanceLastOperationGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/service_instances/{instance_id}/last_operation", configuration.base_path, instance_id=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = service_id {
        local_var_req_builder = local_var_req_builder.query(&[("service_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = plan_id {
        local_var_req_builder = local_var_req_builder.query(&[("plan_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = operation {
        local_var_req_builder = local_var_req_builder.query(&[("operation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceInstanceLastOperationGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn service_instance_provision(configuration: &configuration::Configuration, x_broker_api_version: &str, instance_id: &str, service_instance_provision_request_body: crate::models::ServiceInstanceProvisionRequestBody, x_broker_api_originating_identity: Option<&str>, accepts_incomplete: Option<bool>) -> Result<crate::models::ServiceInstanceProvisionResponse, Error<ServiceInstanceProvisionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/service_instances/{instance_id}", configuration.base_path, instance_id=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = accepts_incomplete {
        local_var_req_builder = local_var_req_builder.query(&[("accepts_incomplete", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
    if let Some(local_var_param_value) = x_broker_api_originating_identity {
        local_var_req_builder = local_var_req_builder.header("X-Broker-API-Originating-Identity", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&service_instance_provision_request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceInstanceProvisionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn service_instance_update(configuration: &configuration::Configuration, x_broker_api_version: &str, instance_id: &str, service_instance_update_request_body: crate::models::ServiceInstanceUpdateRequestBody, x_broker_api_originating_identity: Option<&str>, accepts_incomplete: Option<bool>) -> Result<serde_json::Value, Error<ServiceInstanceUpdateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v2/service_instances/{instance_id}", configuration.base_path, instance_id=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = accepts_incomplete {
        local_var_req_builder = local_var_req_builder.query(&[("accepts_incomplete", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Broker-API-Version", x_broker_api_version.to_string());
    if let Some(local_var_param_value) = x_broker_api_originating_identity {
        local_var_req_builder = local_var_req_builder.header("X-Broker-API-Originating-Identity", local_var_param_value.to_string());
    }
    if let Some(ref local_var_auth_conf) = configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&service_instance_update_request_body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceInstanceUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

