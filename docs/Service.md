# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**id** | **String** |  | 
**description** | **String** |  | 
**tags** | Option<**Vec<String>**> |  | [optional]
**requires** | Option<**Vec<String>**> |  | [optional]
**bindable** | **bool** |  | 
**metadata** | Option<[**serde_json::Value**](.md)> | See [Service Metadata Conventions](https://github.com/openservicebrokerapi/servicebroker/blob/master/profile.md#service-metadata) for more details. | [optional]
**dashboard_client** | Option<[**crate::models::DashboardClient**](DashboardClient.md)> |  | [optional]
**binding_rotatable** | Option<**bool**> |  | [optional]
**plan_updateable** | Option<**bool**> |  | [optional]
**plans** | [**Vec<crate::models::Plan>**](Plan.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


