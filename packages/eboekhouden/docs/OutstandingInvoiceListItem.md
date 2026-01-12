# OutstandingInvoiceListItem

## Properties

| Name                   | Type                                 | Description                                                                                                              | Notes      |
| ---------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ | ---------- |
| **date**               | Option<[**serde_json::Value**](.md)> | Invoice date.                                                                                                            | [optional] |
| **mutation_id**        | Option<**i32**>                      | The ID of the invoice mutation.                                                                                          | [optional] |
| **invoice_number**     | Option<**String**>                   | The invoice number. To keep your code ready for future API versions it is recommended to use `mutationId` instead.       | [optional] |
| **relation_id**        | Option<**i32**>                      | The ID of the relation.                                                                                                  | [optional] |
| **relation_code**      | Option<**String**>                   | The code of the relation. To keep your code ready for future API versions it is recommended to use `relationId` instead. | [optional] |
| **company**            | Option<**String**>                   | Company name.                                                                                                            | [optional] |
| **total_amount**       | Option<**f64**>                      | The invoice amount, including VAT.                                                                                       | [optional] |
| **paid_amount**        | Option<**f64**>                      | The amount received or paid.                                                                                             | [optional] |
| **outstanding_amount** | Option<**f64**>                      | The amount yet to be received or paid.                                                                                   | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
