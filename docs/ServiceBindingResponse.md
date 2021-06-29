# ServiceBindingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metadata** | Option<[**crate::models::ServiceBindingMetadata**](ServiceBindingMetadata.md)> |  | [optional]
**credentials** | Option<[**serde_json::Value**](.md)> |  | [optional]
**syslog_drain_url** | Option<**String**> |  | [optional]
**route_service_url** | Option<**String**> |  | [optional]
**volume_mounts** | Option<[**Vec<crate::models::ServiceBindingVolumeMount>**](ServiceBindingVolumeMount.md)> |  | [optional]
**endpoints** | Option<[**Vec<crate::models::ServiceBindingEndpoint>**](ServiceBindingEndpoint.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


