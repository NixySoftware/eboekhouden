# \RelationApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                                | HTTP request                | Description |
| ----------------------------------------------------- | --------------------------- | ----------- |
| [**create_relation**](RelationApi.md#create_relation) | **POST** /v1/relation       |
| [**get_relation**](RelationApi.md#get_relation)       | **GET** /v1/relation/{id}   |
| [**get_relations**](RelationApi.md#get_relations)     | **GET** /v1/relation        |
| [**update_relation**](RelationApi.md#update_relation) | **PATCH** /v1/relation/{id} |

## create_relation

> models::CreatedRelation create_relation(create_relation)

Create a new relation.

### Parameters

| Name                | Type                                            | Description | Required | Notes |
| ------------------- | ----------------------------------------------- | ----------- | -------- | ----- |
| **create_relation** | Option<[**CreateRelation**](CreateRelation.md)> |             |          |

### Return type

[**models::CreatedRelation**](CreatedRelation.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_relation

> models::Relation get_relation(id)

Get a relation.

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

[**models::Relation**](Relation.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_relations

> models::RelationList get_relations(limit, offset, code, r#type, email, name, contact, city)

Get all relations.

### Parameters

| Name        | Type               | Description                                             | Required | Notes |
| ----------- | ------------------ | ------------------------------------------------------- | -------- | ----- |
| **limit**   | Option<**i32**>    | The number of items to retrieve.                        |          |
| **offset**  | Option<**i32**>    | The number of items to skip.                            |          |
| **code**    | Option<**String**> | The code of the relation.                               |          |
| **r#type**  | Option<**String**> | Business (`B`) or Private (`P`).                        |          |
| **email**   | Option<**String**> | Only retrieves relations with this e-mailadress.        |          |
| **name**    | Option<**String**> | Only retrieves relations with this (company) name.      |          |
| **contact** | Option<**String**> | Only retrieves relations with this contact information. |          |
| **city**    | Option<**String**> | Only retrieves relations from this primary city.        |          |

### Return type

[**models::RelationList**](RelationList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_relation

> update_relation(id, patch_relation)

Update an existing relation.

### Parameters

| Name               | Type                                          | Description | Required   | Notes |
| ------------------ | --------------------------------------------- | ----------- | ---------- | ----- |
| **id**             | **i32**                                       |             | [required] |
| **patch_relation** | Option<[**PatchRelation**](PatchRelation.md)> |             |            |

### Return type

(empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
