// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Filter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.field_name {
        object.key("FieldName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.operator {
        object.key("Operator").string(var_2.as_str());
    }
    if let Some(var_3) = &input.value {
        object.key("Value").string(var_3.as_str());
    }
    Ok(())
}
