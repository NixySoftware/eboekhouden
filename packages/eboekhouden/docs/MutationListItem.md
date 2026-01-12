# MutationListItem

## Properties

| Name               | Type                                                | Description                        | Notes      |
| ------------------ | --------------------------------------------------- | ---------------------------------- | ---------- |
| **id**             | Option<**i32**>                                     | The id of the mutation             | [optional] |
| **r#type**         | Option<[**models::MutationType**](MutationType.md)> |                                    | [optional] |
| **date**           | Option<[**serde_json::Value**](.md)>                | The date of the mutation           | [optional] |
| **invoice_number** | Option<**String**>                                  | The invoice number of the mutation | [optional] |
| **ledger_id**      | Option<**i32**>                                     | The ledger id of the mutation.     | [optional] |
| **amount**         | Option<**f64**>                                     | The amount of the mutation.        | [optional] |
| **entry_number**   | Option<**String**>                                  | The entry number of the mutation.  | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
