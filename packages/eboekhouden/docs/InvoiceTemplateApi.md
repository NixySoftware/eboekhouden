# \InvoiceTemplateApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                                                   | HTTP request                | Description |
| ------------------------------------------------------------------------ | --------------------------- | ----------- |
| [**get_invoice_templates**](InvoiceTemplateApi.md#get_invoice_templates) | **GET** /v1/invoicetemplate |

## get_invoice_templates

> models::InvoiceTemplateList get_invoice_templates(limit, offset, name, r#type, active)

Get all invoice templates.

### Parameters

| Name       | Type               | Description                                    | Required | Notes |
| ---------- | ------------------ | ---------------------------------------------- | -------- | ----- |
| **limit**  | Option<**i32**>    | The number of items to retrieve.               |          |
| **offset** | Option<**i32**>    | The number of items to skip.                   |          |
| **name**   | Option<**String**> | Name of the invoice template.                  |          |
| **r#type** | Option<**String**> | Template type. Editor (`E`) or advanced (`A`). |          |
| **active** | Option<**bool**>   | Whether the invoice template is active.        |          |

### Return type

[**models::InvoiceTemplateList**](InvoiceTemplateList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
