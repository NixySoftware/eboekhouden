# LedgerListItem

## Properties

| Name            | Type                                        | Description                                                                                                                                          | Notes      |
| --------------- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| **id**          | Option<**i32**>                             | The ID of the ledger.                                                                                                                                | [optional] |
| **code**        | Option<**String**>                          | The code of the ledger.                                                                                                                              | [optional] |
| **description** | Option<**String**>                          | The description of the ledger.                                                                                                                       | [optional] |
| **category**    | Option<[**models::Category**](Category.md)> |                                                                                                                                                      | [optional] |
| **group**       | Option<**String**>                          | The group of the ledger. To keep your code ready for future API versions it is recommended to retrieve the group with `GET /v1/ledger/{id}` instead. | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
