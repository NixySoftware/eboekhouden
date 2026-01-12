# \AdministrationApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                                                            | HTTP request                      | Description |
| --------------------------------------------------------------------------------- | --------------------------------- | ----------- |
| [**get_administrations**](AdministrationApi.md#get_administrations)               | **GET** /v1/administration        |
| [**get_linked_administrations**](AdministrationApi.md#get_linked_administrations) | **GET** /v1/administration/linked |

## get_administrations

> models::AdministrationList get_administrations(limit, offset)

Get all administrations managed by the authorized accountant.

### Parameters

| Name       | Type            | Description                      | Required | Notes |
| ---------- | --------------- | -------------------------------- | -------- | ----- |
| **limit**  | Option<**i32**> | The number of items to retrieve. |          |
| **offset** | Option<**i32**> | The number of items to skip.     |          |

### Return type

[**models::AdministrationList**](AdministrationList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_linked_administrations

> models::AdministrationList get_linked_administrations(limit, offset)

Get all administrations that are linked to the authorized administration.<br/>Please note that this endpoint will always return administrations linked to the administration of the user that created the API credentials.

### Parameters

| Name       | Type            | Description                      | Required | Notes |
| ---------- | --------------- | -------------------------------- | -------- | ----- |
| **limit**  | Option<**i32**> | The number of items to retrieve. |          |
| **offset** | Option<**i32**> | The number of items to skip.     |          |

### Return type

[**models::AdministrationList**](AdministrationList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
