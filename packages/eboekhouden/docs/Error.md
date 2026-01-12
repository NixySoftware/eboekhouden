# Error

## Properties

| Name              | Type                    | Description                                                                             | Notes      |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------- | ---------- |
| **errors**        | Option<**Vec<String>**> | Collection of multiple validation errors. Only present when the Type is \"validation\". | [optional] |
| **r#type**        | **String**              | The error type, e.g. \"validation\" or \"security\".                                    |
| **property_name** | Option<**String**>      | The property related to this exception.                                                 | [optional] |
| **code**          | Option<**String**>      | The error code.                                                                         | [optional] |
| **title**         | **String**              | The short error message.                                                                |
| **message**       | Option<**String**>      | Additional error info. Not always present.                                              | [optional] |
| **status**        | **i32**                 | The HTTP status.                                                                        |
| **trace_id**      | Option<**String**>      | Unique trace to identify this error. Not always present.                                | [optional] |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
