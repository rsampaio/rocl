# ServiceBindingRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context** | Option<[**serde_json::Value**](.md)> | See [Context Conventions](https://github.com/openservicebrokerapi/servicebroker/blob/master/profile.md#context-object) for more details. | [optional]
**service_id** | **String** |  | 
**plan_id** | **String** |  | 
**app_guid** | Option<**String**> |  | [optional]
**bind_resource** | Option<[**crate::models::ServiceBindingResouceObject**](ServiceBindingResouceObject.md)> |  | [optional]
**parameters** | Option<[**serde_json::Value**](.md)> |  | [optional]
**predecessor_binding_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


