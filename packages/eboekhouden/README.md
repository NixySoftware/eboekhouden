# Rust API client for e-Boekhouden

Rust OpenAPI client for [e-Boekhouden](https://www.e-boekhouden.nl/).

For more information, visit https://www.e-boekhouden.nl/koppelingen/api.

## Installation

```shell
cargo add eboekhouden
```

## Documentation

Documentation for is available on [Docs.rs](https://docs.rs/eboekhouden/latest/eboekhouden/).

### Documentation for API Endpoints

All URIs are relative to _https://api.e-boekhouden.nl_

| Class                | Method                                                                                 | HTTP request                             | Description |
| -------------------- | -------------------------------------------------------------------------------------- | ---------------------------------------- | ----------- |
| _AdministrationApi_  | [**get_administrations**](docs/AdministrationApi.md#get_administrations)               | **GET** /v1/administration               |
| _AdministrationApi_  | [**get_linked_administrations**](docs/AdministrationApi.md#get_linked_administrations) | **GET** /v1/administration/linked        |
| _CostCenterApi_      | [**create_cost_center**](docs/CostCenterApi.md#create_cost_center)                     | **POST** /v1/costcenter                  |
| _CostCenterApi_      | [**delete_cost_center**](docs/CostCenterApi.md#delete_cost_center)                     | **DELETE** /v1/costcenter/{id}           |
| _CostCenterApi_      | [**get_cost_center**](docs/CostCenterApi.md#get_cost_center)                           | **GET** /v1/costcenter/{id}              |
| _CostCenterApi_      | [**get_cost_centers**](docs/CostCenterApi.md#get_cost_centers)                         | **GET** /v1/costcenter                   |
| _CostCenterApi_      | [**update_cost_center**](docs/CostCenterApi.md#update_cost_center)                     | **PATCH** /v1/costcenter/{id}            |
| _EmailTemplateApi_   | [**get_email_templates**](docs/EmailTemplateApi.md#get_email_templates)                | **GET** /v1/emailtemplate                |
| _InvoiceApi_         | [**create_invoice**](docs/InvoiceApi.md#create_invoice)                                | **POST** /v1/invoice                     |
| _InvoiceApi_         | [**get_invoice**](docs/InvoiceApi.md#get_invoice)                                      | **GET** /v1/invoice/{id}                 |
| _InvoiceApi_         | [**get_invoices**](docs/InvoiceApi.md#get_invoices)                                    | **GET** /v1/invoice                      |
| _InvoiceTemplateApi_ | [**get_invoice_templates**](docs/InvoiceTemplateApi.md#get_invoice_templates)          | **GET** /v1/invoicetemplate              |
| _LedgerApi_          | [**create_ledger**](docs/LedgerApi.md#create_ledger)                                   | **POST** /v1/ledger                      |
| _LedgerApi_          | [**get_balance**](docs/LedgerApi.md#get_balance)                                       | **GET** /v1/ledger/{id}/balance          |
| _LedgerApi_          | [**get_ledger**](docs/LedgerApi.md#get_ledger)                                         | **GET** /v1/ledger/{id}                  |
| _LedgerApi_          | [**get_ledgers**](docs/LedgerApi.md#get_ledgers)                                       | **GET** /v1/ledger                       |
| _LedgerApi_          | [**update_ledger**](docs/LedgerApi.md#update_ledger)                                   | **PATCH** /v1/ledger/{id}                |
| _MemberApi_          | [**create_member**](docs/MemberApi.md#create_member)                                   | **POST** /v1/member                      |
| _MemberApi_          | [**get_member**](docs/MemberApi.md#get_member)                                         | **GET** /v1/member/{id}                  |
| _MemberApi_          | [**get_members**](docs/MemberApi.md#get_members)                                       | **GET** /v1/member                       |
| _MemberApi_          | [**update_member**](docs/MemberApi.md#update_member)                                   | **PATCH** /v1/member/{id}                |
| _MutationApi_        | [**create_mutation**](docs/MutationApi.md#create_mutation)                             | **POST** /v1/mutation                    |
| _MutationApi_        | [**get_mutation**](docs/MutationApi.md#get_mutation)                                   | **GET** /v1/mutation/{id}                |
| _MutationApi_        | [**get_mutations**](docs/MutationApi.md#get_mutations)                                 | **GET** /v1/mutation                     |
| _MutationApi_        | [**get_outstanding_invoices**](docs/MutationApi.md#get_outstanding_invoices)           | **GET** /v1/mutation/invoice/outstanding |
| _ProductApi_         | [**create_product**](docs/ProductApi.md#create_product)                                | **POST** /v1/product                     |
| _ProductApi_         | [**delete_product**](docs/ProductApi.md#delete_product)                                | **DELETE** /v1/product/{id}              |
| _ProductApi_         | [**get_product**](docs/ProductApi.md#get_product)                                      | **GET** /v1/product/{id}                 |
| _ProductApi_         | [**get_product_group**](docs/ProductApi.md#get_product_group)                          | **GET** /v1/product/groups               |
| _ProductApi_         | [**get_products**](docs/ProductApi.md#get_products)                                    | **GET** /v1/product                      |
| _ProductApi_         | [**update_product**](docs/ProductApi.md#update_product)                                | **PATCH** /v1/product/{id}               |
| _RelationApi_        | [**create_relation**](docs/RelationApi.md#create_relation)                             | **POST** /v1/relation                    |
| _RelationApi_        | [**get_relation**](docs/RelationApi.md#get_relation)                                   | **GET** /v1/relation/{id}                |
| _RelationApi_        | [**get_relations**](docs/RelationApi.md#get_relations)                                 | **GET** /v1/relation                     |
| _RelationApi_        | [**update_relation**](docs/RelationApi.md#update_relation)                             | **PATCH** /v1/relation/{id}              |
| _SessionApi_         | [**end_session**](docs/SessionApi.md#end_session)                                      | **DELETE** /v1/session                   |
| _SessionApi_         | [**start_session**](docs/SessionApi.md#start_session)                                  | **POST** /v1/session                     |
| _UnitApi_            | [**get_units**](docs/UnitApi.md#get_units)                                             | **GET** /v1/unit                         |

### Documentation For Models

- [AdministrationList](docs/AdministrationList.md)
- [AdministrationListItem](docs/AdministrationListItem.md)
- [Category](docs/Category.md)
- [CostCenter](docs/CostCenter.md)
- [CostCenter1](docs/CostCenter1.md)
- [CostCenterCreateResponse](docs/CostCenterCreateResponse.md)
- [CostCenterList](docs/CostCenterList.md)
- [CostCenterResponse](docs/CostCenterResponse.md)
- [CostCenterSearchResponse](docs/CostCenterSearchResponse.md)
- [CreateInvoice](docs/CreateInvoice.md)
- [CreateInvoiceDebit](docs/CreateInvoiceDebit.md)
- [CreateInvoiceEmail](docs/CreateInvoiceEmail.md)
- [CreateInvoiceItem](docs/CreateInvoiceItem.md)
- [CreateInvoiceMutation](docs/CreateInvoiceMutation.md)
- [CreateLedger](docs/CreateLedger.md)
- [CreateMutation](docs/CreateMutation.md)
- [CreateMutationRow](docs/CreateMutationRow.md)
- [CreateProduct](docs/CreateProduct.md)
- [CreateProduct1](docs/CreateProduct1.md)
- [CreateRelation](docs/CreateRelation.md)
- [CreatedInvoice](docs/CreatedInvoice.md)
- [CreatedLedger](docs/CreatedLedger.md)
- [CreatedMutation](docs/CreatedMutation.md)
- [CreatedRelation](docs/CreatedRelation.md)
- [EmailTemplateList](docs/EmailTemplateList.md)
- [EmailTemplateListItem](docs/EmailTemplateListItem.md)
- [Error](docs/Error.md)
- [Invoice](docs/Invoice.md)
- [InvoiceItem](docs/InvoiceItem.md)
- [InvoiceList](docs/InvoiceList.md)
- [InvoiceListItem](docs/InvoiceListItem.md)
- [InvoiceTemplateList](docs/InvoiceTemplateList.md)
- [InvoiceTemplateListItem](docs/InvoiceTemplateListItem.md)
- [Ledger](docs/Ledger.md)
- [LedgerBalance](docs/LedgerBalance.md)
- [LedgerList](docs/LedgerList.md)
- [LedgerListItem](docs/LedgerListItem.md)
- [List](docs/List.md)
- [List1](docs/List1.md)
- [Member](docs/Member.md)
- [Member1](docs/Member1.md)
- [MembersResponse](docs/MembersResponse.md)
- [MembersResponse1](docs/MembersResponse1.md)
- [Mutation](docs/Mutation.md)
- [MutationList](docs/MutationList.md)
- [MutationListItem](docs/MutationListItem.md)
- [MutationRow](docs/MutationRow.md)
- [MutationType](docs/MutationType.md)
- [OutstandingInvoiceListItem](docs/OutstandingInvoiceListItem.md)
- [OutstandingInvoicesList](docs/OutstandingInvoicesList.md)
- [PatchLedger](docs/PatchLedger.md)
- [PatchProduct](docs/PatchProduct.md)
- [PatchRelation](docs/PatchRelation.md)
- [Product](docs/Product.md)
- [ProductGroup](docs/ProductGroup.md)
- [ProductList](docs/ProductList.md)
- [ProductListItem](docs/ProductListItem.md)
- [Relation](docs/Relation.md)
- [RelationList](docs/RelationList.md)
- [RelationListItem](docs/RelationListItem.md)
- [SecurityError](docs/SecurityError.md)
- [SecurityErrorType](docs/SecurityErrorType.md)
- [Session](docs/Session.md)
- [SessionRequest](docs/SessionRequest.md)
- [UnitList](docs/UnitList.md)
- [UnitListItem](docs/UnitListItem.md)
- [UpdateSubscriptions](docs/UpdateSubscriptions.md)
- [VatAmount](docs/VatAmount.md)
- [VatCode](docs/VatCode.md)

## License

This project is available under the [MIT license](../../LICENSE.md).
