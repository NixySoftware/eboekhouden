# \LedgerApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                          | HTTP request                    | Description |
| ----------------------------------------------- | ------------------------------- | ----------- |
| [**create_ledger**](LedgerApi.md#create_ledger) | **POST** /v1/ledger             |
| [**get_balance**](LedgerApi.md#get_balance)     | **GET** /v1/ledger/{id}/balance |
| [**get_ledger**](LedgerApi.md#get_ledger)       | **GET** /v1/ledger/{id}         |
| [**get_ledgers**](LedgerApi.md#get_ledgers)     | **GET** /v1/ledger              |
| [**update_ledger**](LedgerApi.md#update_ledger) | **PATCH** /v1/ledger/{id}       |

## create_ledger

> models::CreatedLedger create_ledger(create_ledger)

Create a new ledger.

### Parameters

| Name              | Type                                        | Description | Required | Notes |
| ----------------- | ------------------------------------------- | ----------- | -------- | ----- |
| **create_ledger** | Option<[**CreateLedger**](CreateLedger.md)> |             |          |

### Return type

[**models::CreatedLedger**](CreatedLedger.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_balance

> models::LedgerBalance get_balance(id, cost_center_id, from, to)

Get the balance on the given ledger with optional filters.

### Parameters

| Name               | Type               | Description                                                                                                                                       | Required   | Notes |
| ------------------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | ----- |
| **id**             | **i32**            |                                                                                                                                                   | [required] |
| **cost_center_id** | Option<**i32**>    | The ID of the cost center.                                                                                                                        |            |
| **from**           | Option<**String**> | Show the balance starting from this date. When provided, the resulting balance is the difference over the period, rather than the actual balance. |            |
| **to**             | Option<**String**> | Shows the active balance at this date. This date is inclusive.                                                                                    |            |

### Return type

[**models::LedgerBalance**](LedgerBalance.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_ledger

> models::Ledger get_ledger(id)

Get a ledger.

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

[**models::Ledger**](Ledger.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_ledgers

> models::LedgerList get_ledgers(limit, offset, code, category)

Get all ledgers.

### Parameters

| Name         | Type               | Description                      | Required | Notes |
| ------------ | ------------------ | -------------------------------- | -------- | ----- |
| **limit**    | Option<**i32**>    | The number of items to retrieve. |          |
| **offset**   | Option<**i32**>    | The number of items to skip.     |          |
| **code**     | Option<**String**> | The code of the ledger.          |          |
| **category** | Option<**String**> | The category of the ledger.      |          |

### Return type

[**models::LedgerList**](LedgerList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_ledger

> update_ledger(id, patch_ledger)

Update a ledger.

### Parameters

| Name             | Type                                      | Description | Required   | Notes |
| ---------------- | ----------------------------------------- | ----------- | ---------- | ----- |
| **id**           | **i32**                                   |             | [required] |
| **patch_ledger** | Option<[**PatchLedger**](PatchLedger.md)> |             |            |

### Return type

(empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
