# Plan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**metadata** | Option<[**serde_json::Value**](.md)> | See [Service Metadata Conventions](https://github.com/openservicebrokerapi/servicebroker/blob/master/profile.md#service-metadata) for more details. | [optional]
**maintenance_info** | Option<[**crate::models::MaintenanceInfo**](MaintenanceInfo.md)> |  | [optional]
**free** | Option<**bool**> |  | [optional][default to true]
**bindable** | Option<**bool**> |  | [optional]
**schemas** | Option<[**crate::models::Schemas**](Schemas.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


