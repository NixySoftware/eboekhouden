# CreatedInvoice

## Properties

| Name               | Type               | Description                                                                                                                                                                                                                                         | Notes      |
| ------------------ | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| **id**             | Option<**i32**>    | The ID of the invoice.                                                                                                                                                                                                                              | [optional] |
| **invoice_number** | Option<**String**> | The invoice number.                                                                                                                                                                                                                                 | [optional] |
| **in_ex_vat**      | Option<**String**> | Indicates if the price per unit of the sub items on the invoice is including (`IN`) or excluding (`EX`) VAT.<br><div><h3>Error codes</h3><table><tr><td><a href=\"#FACT_010\">FACT_010</a></td><td>Invoice VAT must be IN or EX.</td></table></div> | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
