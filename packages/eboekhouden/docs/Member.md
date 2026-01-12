# Member

## Properties

| Name                           | Type                                 | Description                                         | Notes      |
| ------------------------------ | ------------------------------------ | --------------------------------------------------- | ---------- |
| **id**                         | Option<**i32**>                      | The ID of the member.                               | [optional] |
| **member_number**              | Option<**String**>                   | The number of the member.                           | [optional] |
| **name**                       | Option<**String**>                   | The (company) name of the member.                   | [optional] |
| **salutation**                 | Option<**String**>                   | The salutation of the member.                       | [optional] |
| **gender**                     | Option<**String**>                   | Male (`m`), female (`v`) or department ('a')        | [optional] |
| **address**                    | Option<**String**>                   | The primary address of the member.                  | [optional] |
| **postal_code**                | Option<**String**>                   | The primary postal code of the member.              | [optional] |
| **city**                       | Option<**String**>                   | The primary city of the member.                     | [optional] |
| **country**                    | Option<**String**>                   | The primary country of the member.                  | [optional] |
| **phone_number**               | Option<**String**>                   | The phone number of the member.                     | [optional] |
| **mobile_phone_number**        | Option<**String**>                   | The mobile phone number of the member.              | [optional] |
| **fax_number**                 | Option<**String**>                   | The fax number of the member.                       | [optional] |
| **email_address**              | Option<**String**>                   | The email address of the member.                    | [optional] |
| **email_address_invoice**      | Option<**String**>                   | The invoice email address of the member.            | [optional] |
| **email_address_reminder**     | Option<**String**>                   | The reminder email address of the member.           | [optional] |
| **note**                       | Option<**String**>                   | The note of the member.                             | [optional] |
| **term_of_payment**            | Option<**i32**>                      | The payment term of the member.                     | [optional] |
| **iban**                       | Option<**String**>                   | The IBAN number of the member.                      | [optional] |
| **bic**                        | Option<**String**>                   | The BIC number of the member.                       | [optional] |
| **free_text1**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text2**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text3**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text4**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text5**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text6**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text7**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text8**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text9**                 | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **free_text10**                | Option<**String**>                   | Free text field for the member.                     | [optional] |
| **do_not_receive_newsletters** | Option<**i32**>                      | Whether the member will receive newsletters or not. | [optional] |
| **ledger_id**                  | Option<**i32**>                      | The ledger ID of this member.                       | [optional] |
| **mandate**                    | Option<**bool**>                     |                                                     | [optional] |
| **mandate_type**               | Option<**String**>                   | one-time (`E`) or continuous (`D`)                  | [optional] |
| **mandate_id**                 | Option<**String**>                   | The mandate ID of this member.                      | [optional] |
| **mandate_signed_date**        | Option<[**serde_json::Value**](.md)> | The signing date of the mandate for this member.    | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
