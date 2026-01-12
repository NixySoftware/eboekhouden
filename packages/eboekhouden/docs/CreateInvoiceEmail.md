# CreateInvoiceEmail

## Properties

| Name            | Type               | Description                                                                                                                                                                                                                                                | Notes                        |
| --------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| **from_email**  | Option<**String**> | Email address of the sender.<br><div><h3>Error codes</h3><table><tr><td><a href=\"#FACT_EMAIL_006\">FACT_EMAIL_006</a></td><td>Sender email is invalid.</td></table></div>                                                                                 | [optional]                   |
| **from_name**   | Option<**String**> | Name of the sender.                                                                                                                                                                                                                                        | [optional]                   |
| **subject**     | Option<**String**> | Email subject.<br><div><h3>Error codes</h3><table><tr><td><a href=\"#FACT_EMAIL_013\">FACT_EMAIL_013</a></td><td>Subject is missing.</td></table></div>                                                                                                    | [optional]                   |
| **body**        | Option<**String**> | Email body. Supports HTML.<br><div><h3>Error codes</h3><table><tr><td><a href=\"#FACT_EMAIL_011\">FACT_EMAIL_011</a></td><td>Email body is too long</td><tr><td><a href=\"#FACT_EMAIL_014\">FACT_EMAIL_014</a></td><td>Body is missing.</td></table></div> | [optional]                   |
| **attach_ubl**  | Option<**bool**>   | Also attach the invoice UBL-file.                                                                                                                                                                                                                          | [optional][default to false] |
| **invoice_ubl** | Option<**bool**>   | Obsolete; please use `attachUbl` instead. This field will be removed in the future.                                                                                                                                                                        | [optional]                   |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
