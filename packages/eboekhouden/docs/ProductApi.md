# \ProductApi

All URIs are relative to *https://api.e-boekhouden.nl*

| Method                                                   | HTTP request                | Description |
| -------------------------------------------------------- | --------------------------- | ----------- |
| [**create_product**](ProductApi.md#create_product)       | **POST** /v1/product        |
| [**delete_product**](ProductApi.md#delete_product)       | **DELETE** /v1/product/{id} |
| [**get_product**](ProductApi.md#get_product)             | **GET** /v1/product/{id}    |
| [**get_product_group**](ProductApi.md#get_product_group) | **GET** /v1/product/groups  |
| [**get_products**](ProductApi.md#get_products)           | **GET** /v1/product         |
| [**update_product**](ProductApi.md#update_product)       | **PATCH** /v1/product/{id}  |

## create_product

> models::CreateProduct1 create_product(create_product)

Create a new product.

### Parameters

| Name               | Type                                          | Description | Required | Notes |
| ------------------ | --------------------------------------------- | ----------- | -------- | ----- |
| **create_product** | Option<[**CreateProduct**](CreateProduct.md)> |             |          |

### Return type

[**models::CreateProduct1**](CreateProduct_1.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## delete_product

> delete_product(id)

Delete a product.

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

## get_product

> models::Product get_product(id)

Get a product.

### Parameters

| Name   | Type    | Description | Required   | Notes |
| ------ | ------- | ----------- | ---------- | ----- |
| **id** | **i32** |             | [required] |

### Return type

[**models::Product**](Product.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_product_group

> models::List1 get_product_group()

Get all product groups.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::List1**](List_1.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## get_products

> models::ProductList get_products(limit, offset, code, group_code)

Get all products.

### Parameters

| Name           | Type               | Description                      | Required | Notes |
| -------------- | ------------------ | -------------------------------- | -------- | ----- |
| **limit**      | Option<**i32**>    | The number of items to retrieve. |          |
| **offset**     | Option<**i32**>    | The number of items to skip.     |          |
| **code**       | Option<**String**> | The code of the product.         |          |
| **group_code** | Option<**String**> | The product group code.          |          |

### Return type

[**models::ProductList**](ProductList.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## update_product

> update_product(id, patch_product)

Update a product.

### Parameters

| Name              | Type                                        | Description | Required   | Notes |
| ----------------- | ------------------------------------------- | ----------- | ---------- | ----- |
| **id**            | **i32**                                     |             | [required] |
| **patch_product** | Option<[**PatchProduct**](PatchProduct.md)> |             |            |

### Return type

(empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
