# \MutationApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                                                  | HTTP request                             | Description |
| ----------------------------------------------------------------------- | ---------------------------------------- | ----------- |
| [**create_mutation**](MutationApi.md#create_mutation)                   | **POST** /v1/mutation                    |
| [**get_mutation**](MutationApi.md#get_mutation)                         | **GET** /v1/mutation/{id}                |
| [**get_mutations**](MutationApi.md#get_mutations)                       | **GET** /v1/mutation                     |
| [**get_outstanding_invoices**](MutationApi.md#get_outstanding_invoices) | **GET** /v1/mutation/invoice/outstanding |

## create_mutation

> models::CreatedMutation create_mutation(create_mutation)

Create a new mutation.

### Parameters

| Name                | Type                                            | Description | Required | Notes |
| ------------------- | ----------------------------------------------- | ----------- | -------- | ----- |
| **create_mutation** | Option<[**CreateMutation**](CreateMutation.md)> |             |          |

### Return type

[**models::CreatedMutation**](CreatedMutation.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_mutation

> models::Mutation get_mutation(id)

Get a mutation by id.

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

[**models::Mutation**](Mutation.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_mutations

> models::MutationList get_mutations(limit, offset, r#type, id, description, invoice_number, date)

Get all mutations.

### Parameters

| Name               | Type               | Description                                        | Required | Notes |
| ------------------ | ------------------ | -------------------------------------------------- | -------- | ----- |
| **limit**          | Option<**i32**>    | The number of items to retrieve.                   |          |
| **offset**         | Option<**i32**>    | The number of items to skip.                       |          |
| **r#type**         | Option<**i32**>    | Only retrieves mutations of this type.             |          |
| **id**             | Option<**i32**>    | Only retrieves mutations with this number.         |          |
| **description**    | Option<**String**> | Only retrieves mutations with this description.    |          |
| **invoice_number** | Option<**String**> | Only retrieves mutations with this invoice number. |          |
| **date**           | Option<**String**> | Only retrieves mutations with this date.           |          |

### Return type

[**models::MutationList**](MutationList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_outstanding_invoices

> models::OutstandingInvoicesList get_outstanding_invoices(cred_deb, limit, offset, invoice_number)

Get all outstanding invoices.

### Parameters

| Name               | Type               | Description                                | Required   | Notes |
| ------------------ | ------------------ | ------------------------------------------ | ---------- | ----- |
| **cred_deb**       | **String**         | Retrieve creditors (`C`) or debtors (`D`). | [required] |
| **limit**          | Option<**i32**>    | The number of items to retrieve.           |            |
| **offset**         | Option<**i32**>    | The number of items to skip.               |            |
| **invoice_number** | Option<**String**> | Only retrieve invoices with this number.   |            |

### Return type

[**models::OutstandingInvoicesList**](OutstandingInvoicesList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
