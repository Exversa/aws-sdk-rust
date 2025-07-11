// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_describe_custom_key_stores_output_output_next_marker(
    input: &crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_aliases_output_output_next_marker(
    input: &crate::operation::list_aliases::ListAliasesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_grants_output_output_next_marker(
    input: &crate::operation::list_grants::ListGrantsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_key_policies_output_output_next_marker(
    input: &crate::operation::list_key_policies::ListKeyPoliciesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_key_rotations_output_output_next_marker(
    input: &crate::operation::list_key_rotations::ListKeyRotationsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_keys_output_output_next_marker(
    input: &crate::operation::list_keys::ListKeysOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_resource_tags_output_output_next_marker(
    input: &crate::operation::list_resource_tags::ListResourceTagsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_retirable_grants_output_output_next_marker(
    input: &crate::operation::list_retirable_grants::ListRetirableGrantsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_marker {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_describe_custom_key_stores_output_output_custom_key_stores(
    input: crate::operation::describe_custom_key_stores::DescribeCustomKeyStoresOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CustomKeyStoresListEntry>> {
    let input = input.custom_key_stores?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_aliases_output_output_aliases(
    input: crate::operation::list_aliases::ListAliasesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AliasListEntry>> {
    let input = input.aliases?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_grants_output_output_grants(
    input: crate::operation::list_grants::ListGrantsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::GrantListEntry>> {
    let input = input.grants?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_key_policies_output_output_policy_names(
    input: crate::operation::list_key_policies::ListKeyPoliciesOutput,
) -> ::std::option::Option<::std::vec::Vec<::std::string::String>> {
    let input = input.policy_names?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_key_rotations_output_output_rotations(
    input: crate::operation::list_key_rotations::ListKeyRotationsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::RotationsListEntry>> {
    let input = input.rotations?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_keys_output_output_keys(
    input: crate::operation::list_keys::ListKeysOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::KeyListEntry>> {
    let input = input.keys?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_resource_tags_output_output_tags(
    input: crate::operation::list_resource_tags::ListResourceTagsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
    let input = input.tags?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_retirable_grants_output_output_grants(
    input: crate::operation::list_retirable_grants::ListRetirableGrantsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::GrantListEntry>> {
    let input = input.grants?;
    ::std::option::Option::Some(input)
}
