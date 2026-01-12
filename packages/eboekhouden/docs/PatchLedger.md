# PatchLedger

## Properties

| Name            | Type                                        | Description                                                                                                                                                                                                                                     | Notes      |
| --------------- | ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| **code**        | Option<**String**>                          | The code of the ledger.<br><div><h3>Error codes</h3><table><tr><td><a href=\"#LEDG_001\">LEDG_001</a></td><td>Code is missing.</td><tr><td><a href=\"#LEDG_002\">LEDG_002</a></td><td>Code is too long.</td></table></div>                      | [optional] |
| **description** | Option<**String**>                          | The description of the ledger.<br><div><h3>Error codes</h3><table><tr><td><a href=\"#LEDG_003\">LEDG_003</a></td><td>Description is missing.</td><tr><td><a href=\"#LEDG_004\">LEDG_004</a></td><td>Description is too long.</td></table></div> | [optional] |
| **category**    | Option<[**models::Category**](Category.md)> |                                                                                                                                                                                                                                                 | [optional] |
| **group**       | Option<**String**>                          | The group of the ledger.<br><div><h3>Error codes</h3><table><tr><td><a href=\"#LEDG_007\">LEDG_007</a></td><td>Group is missing.</td></table></div>                                                                                             | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
