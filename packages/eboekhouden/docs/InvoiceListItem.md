# InvoiceListItem

## Properties

| Name                | Type                                 | Description                                                     | Notes      |
| ------------------- | ------------------------------------ | --------------------------------------------------------------- | ---------- |
| **id**              | Option<**i32**>                      | The ID of the invoice.                                          | [optional] |
| **invoice_number**  | Option<**String**>                   | The invoice number.                                             | [optional] |
| **relation_id**     | Option<**i32**>                      | The ID of a relation.                                           | [optional] |
| **date**            | Option<[**serde_json::Value**](.md)> | Invoice date.                                                   | [optional] |
| **reference**       | Option<**String**>                   | Reference of the invoice.                                       | [optional] |
| **text**            | Option<**String**>                   | Text of the invoice.                                            | [optional] |
| **url_pdf_file**    | Option<**String**>                   | A shareable url to download the invoice as a PDF file.          | [optional] |
| **total_excl**      | Option<**f64**>                      | The invoice amount, excluding VAT.                              | [optional] |
| **total_amount**    | Option<**f64**>                      | The invoice amount, including VAT.                              | [optional] |
| **vat_amount**      | Option<**f64**>                      | The VAT amount.                                                 | [optional] |
| **term_of_payment** | Option<**i32**>                      | Term of payment from the invoice date, in days.                 | [optional] |
| **template_id**     | Option<**i32**>                      | The ID of an invoice template.                                  | [optional] |
| **print**           | Option<**bool**>                     | Determines whether the invoice should be prepared for printing. | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
