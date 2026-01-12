# Mutation

## Properties

| Name                | Type                                                   | Description                               | Notes      |
| ------------------- | ------------------------------------------------------ | ----------------------------------------- | ---------- |
| **id**              | Option<**i64**>                                        | The ID (number) of the mutation.          | [optional] |
| **r#type**          | Option<[**models::MutationType**](MutationType.md)>    |                                           | [optional] |
| **date**            | Option<[**String**](string.md)>                        |                                           | [optional] |
| **description**     | Option<**String**>                                     |                                           | [optional] |
| **term_of_payment** | Option<**i32**>                                        |                                           | [optional] |
| **ledger_id**       | Option<**i32**>                                        |                                           | [optional] |
| **relation_id**     | Option<**i32**>                                        |                                           | [optional] |
| **in_ex_vat**       | Option<**String**>                                     |                                           | [optional] |
| **invoice_number**  | Option<**String**>                                     |                                           | [optional] |
| **entry_number**    | Option<**String**>                                     |                                           | [optional] |
| **rows**            | Option<[**Vec<models::MutationRow>**](MutationRow.md)> |                                           | [optional] |
| **vat**             | Option<[**Vec<models::VatAmount>**](VatAmount.md)>     | VAT totals for this mutation per VAT code | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
