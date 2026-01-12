# \InvoiceApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                             | HTTP request             | Description |
| -------------------------------------------------- | ------------------------ | ----------- |
| [**create_invoice**](InvoiceApi.md#create_invoice) | **POST** /v1/invoice     |
| [**get_invoice**](InvoiceApi.md#get_invoice)       | **GET** /v1/invoice/{id} |
| [**get_invoices**](InvoiceApi.md#get_invoices)     | **GET** /v1/invoice      |

## create_invoice

> models::CreatedInvoice create_invoice(create_invoice)

Create a new invoice.

### Parameters

| Name               | Type                                          | Description | Required | Notes |
| ------------------ | --------------------------------------------- | ----------- | -------- | ----- |
| **create_invoice** | Option<[**CreateInvoice**](CreateInvoice.md)> |             |          |

### Return type

[**models::CreatedInvoice**](CreatedInvoice.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_invoice

> models::Invoice get_invoice(id)

Get an invoice.

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

[**models::Invoice**](Invoice.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_invoices

> models::InvoiceList get_invoices(limit, offset, invoice_number, relation_id, date)

Search for invoices.

### Parameters

| Name               | Type               | Description                                 | Required | Notes |
| ------------------ | ------------------ | ------------------------------------------- | -------- | ----- |
| **limit**          | Option<**i32**>    | The number of items to retrieve.            |          |
| **offset**         | Option<**i32**>    | The number of items to skip.                |          |
| **invoice_number** | Option<**String**> | Only retrieve the invoice with this number. |          |
| **relation_id**    | Option<**i32**>    | Only retrieve invoices from this relation.  |          |
| **date**           | Option<**String**> | Only retrieve invoices on this date.        |          |

### Return type

[**models::InvoiceList**](InvoiceList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
