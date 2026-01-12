# \CostCenterApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                                        | HTTP request                   | Description |
| ------------------------------------------------------------- | ------------------------------ | ----------- |
| [**create_cost_center**](CostCenterApi.md#create_cost_center) | **POST** /v1/costcenter        |
| [**delete_cost_center**](CostCenterApi.md#delete_cost_center) | **DELETE** /v1/costcenter/{id} |
| [**get_cost_center**](CostCenterApi.md#get_cost_center)       | **GET** /v1/costcenter/{id}    |
| [**get_cost_centers**](CostCenterApi.md#get_cost_centers)     | **GET** /v1/costcenter         |
| [**update_cost_center**](CostCenterApi.md#update_cost_center) | **PATCH** /v1/costcenter/{id}  |

## create_cost_center

> models::CostCenterCreateResponse create_cost_center(cost_center)

Create a new cost center.

### Parameters

| Name            | Type                                    | Description | Required | Notes |
| --------------- | --------------------------------------- | ----------- | -------- | ----- |
| **cost_center** | Option<[**CostCenter**](CostCenter.md)> |             |          |

### Return type

[**models::CostCenterCreateResponse**](CostCenterCreateResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_cost_center

> delete_cost_center(id)

Delete a cost center.

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

(empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cost_center

> models::CostCenterResponse get_cost_center(id)

Get a cost center.

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

[**models::CostCenterResponse**](CostCenterResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_cost_centers

> models::CostCenterList get_cost_centers(limit, offset, parent_id, description)

Get all cost centers.

### Parameters

| Name            | Type               | Description                         | Required | Notes |
| --------------- | ------------------ | ----------------------------------- | -------- | ----- |
| **limit**       | Option<**i32**>    | The number of items to retrieve.    |          |
| **offset**      | Option<**i32**>    | The number of items to skip.        |          |
| **parent_id**   | Option<**String**> | The parent ID of the cost center.   |          |
| **description** | Option<**String**> | The description of the cost center. |          |

### Return type

[**models::CostCenterList**](CostCenterList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_cost_center

> update_cost_center(id, cost_center1)

Update a cost center.

### Parameters

| Name             | Type                                      | Description | Required   | Notes |
| ---------------- | ----------------------------------------- | ----------- | ---------- | ----- |
| **id**           | **i32**                                   |             | [required] |
| **cost_center1** | Option<[**CostCenter1**](CostCenter1.md)> |             |            |

### Return type

(empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
