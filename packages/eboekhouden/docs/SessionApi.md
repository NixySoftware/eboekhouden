# \SessionApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                           | HTTP request           | Description |
| ------------------------------------------------ | ---------------------- | ----------- |
| [**end_session**](SessionApi.md#end_session)     | **DELETE** /v1/session |
| [**start_session**](SessionApi.md#start_session) | **POST** /v1/session   |

## end_session

> end_session()

Revokes the session token.

### Parameters

This endpoint does not need any parameter.

### Return type

(empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## start_session

> models::Session start_session(session_request)

Start a new session. The response session token can be used as the `Authorization` header.

### Parameters

| Name                | Type                                            | Description | Required | Notes |
| ------------------- | ----------------------------------------------- | ----------- | -------- | ----- |
| **session_request** | Option<[**SessionRequest**](SessionRequest.md)> |             |          |

### Return type

[**models::Session**](Session.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
