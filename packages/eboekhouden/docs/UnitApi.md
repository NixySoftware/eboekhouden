# \UnitApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                | HTTP request     | Description |
| ------------------------------------- | ---------------- | ----------- |
| [**get_units**](UnitApi.md#get_units) | **GET** /v1/unit |

## get_units

> models::UnitList get_units(limit, offset, singular, plural)

Get all units.

### Parameters

| Name         | Type               | Description                      | Required | Notes |
| ------------ | ------------------ | -------------------------------- | -------- | ----- |
| **limit**    | Option<**i32**>    | The number of items to retrieve. |          |
| **offset**   | Option<**i32**>    | The number of items to skip.     |          |
| **singular** | Option<**String**> | The singular form for the unit.  |          |
| **plural**   | Option<**String**> | The plural form for the unit.    |          |

### Return type

[**models::UnitList**](UnitList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
