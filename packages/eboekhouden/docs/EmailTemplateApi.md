# \EmailTemplateApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                                             | HTTP request              | Description |
| ------------------------------------------------------------------ | ------------------------- | ----------- |
| [**get_email_templates**](EmailTemplateApi.md#get_email_templates) | **GET** /v1/emailtemplate |

## get_email_templates

> models::EmailTemplateList get_email_templates(limit, offset)

Get all email templates.

### Parameters

| Name       | Type            | Description                      | Required | Notes |
| ---------- | --------------- | -------------------------------- | -------- | ----- |
| **limit**  | Option<**i32**> | The number of items to retrieve. |          |
| **offset** | Option<**i32**> | The number of items to skip.     |          |

### Return type

[**models::EmailTemplateList**](EmailTemplateList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
