# \MemberApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                          | HTTP request              | Description |
| ----------------------------------------------- | ------------------------- | ----------- |
| [**create_member**](MemberApi.md#create_member) | **POST** /v1/member       |
| [**get_member**](MemberApi.md#get_member)       | **GET** /v1/member/{id}   |
| [**get_members**](MemberApi.md#get_members)     | **GET** /v1/member        |
| [**update_member**](MemberApi.md#update_member) | **PATCH** /v1/member/{id} |

## create_member

> models::MembersResponse1 create_member(member1)

Create a new member (only available to clubs or associations).

### Parameters

| Name        | Type                              | Description | Required | Notes |
| ----------- | --------------------------------- | ----------- | -------- | ----- |
| **member1** | Option<[**Member1**](Member1.md)> |             |          |

### Return type

[**models::MembersResponse1**](MembersResponse_1.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_member

> models::Member get_member(id)

Get a member (only available to clubs or associations).

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

[**models::Member**](Member.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_members

> models::List get_members(limit, offset, member_number, name, email, city)

Get all members (only available to clubs or associations).

### Parameters

| Name              | Type               | Description                                    | Required | Notes |
| ----------------- | ------------------ | ---------------------------------------------- | -------- | ----- |
| **limit**         | Option<**i32**>    | The number of items to retrieve.               |          |
| **offset**        | Option<**i32**>    | The number of items to skip.                   |          |
| **member_number** | Option<**String**> | The number of the member.                      |          |
| **name**          | Option<**String**> | Only retrieves members with this name.         |          |
| **email**         | Option<**String**> | Only retrieves members with this e-mailadress. |          |
| **city**          | Option<**String**> | Only retrieves members from this city.         |          |

### Return type

[**models::List**](List.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_member

> update_member(id, member1)

Update an existing member (only available to clubs or associations).

### Parameters

| Name        | Type                              | Description | Required   | Notes |
| ----------- | --------------------------------- | ----------- | ---------- | ----- |
| **id**      | **i32**                           |             | [required] |
| **member1** | Option<[**Member1**](Member1.md)> |             |            |

### Return type

(empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
