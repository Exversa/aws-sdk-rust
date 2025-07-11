// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_policy_version::_get_policy_version_output::GetPolicyVersionOutputBuilder;

pub use crate::operation::get_policy_version::_get_policy_version_input::GetPolicyVersionInputBuilder;

impl crate::operation::get_policy_version::builders::GetPolicyVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_policy_version::GetPolicyVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_policy_version::GetPolicyVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_policy_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPolicyVersion`.
///
/// <p>Returns details for the version of a policy. Policies define the permissions for team resources.</p>
/// <p>The protected operation for a service integration might require specific permissions. For more information, see <a href="https://docs.aws.amazon.com/mpa/latest/userguide/mpa-integrations.html">How other services work with Multi-party approval</a> in the <i>Multi-party approval User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPolicyVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_policy_version::builders::GetPolicyVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_policy_version::GetPolicyVersionOutput,
        crate::operation::get_policy_version::GetPolicyVersionError,
    > for GetPolicyVersionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_policy_version::GetPolicyVersionOutput,
            crate::operation::get_policy_version::GetPolicyVersionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPolicyVersionFluentBuilder {
    /// Creates a new `GetPolicyVersionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPolicyVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::get_policy_version::builders::GetPolicyVersionInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_policy_version::GetPolicyVersionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_policy_version::GetPolicyVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_policy_version::GetPolicyVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_policy_version::GetPolicyVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_policy_version::GetPolicyVersionOutput,
        crate::operation::get_policy_version::GetPolicyVersionError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Amazon Resource Name (ARN) for the policy.</p>
    pub fn policy_version_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_version_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) for the policy.</p>
    pub fn set_policy_version_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_version_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) for the policy.</p>
    pub fn get_policy_version_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_version_arn()
    }
}
