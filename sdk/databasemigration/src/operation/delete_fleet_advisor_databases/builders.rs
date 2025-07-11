// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_fleet_advisor_databases::_delete_fleet_advisor_databases_output::DeleteFleetAdvisorDatabasesOutputBuilder;

pub use crate::operation::delete_fleet_advisor_databases::_delete_fleet_advisor_databases_input::DeleteFleetAdvisorDatabasesInputBuilder;

impl crate::operation::delete_fleet_advisor_databases::builders::DeleteFleetAdvisorDatabasesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_fleet_advisor_databases();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteFleetAdvisorDatabases`.
///
/// <important>
/// <p>End of support notice: On May 20, 2026, Amazon Web Services will end support for Amazon Web Services DMS Fleet Advisor;. After May 20, 2026, you will no longer be able to access the Amazon Web Services DMS Fleet Advisor; console or Amazon Web Services DMS Fleet Advisor; resources. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/dms_fleet.advisor-end-of-support.html">Amazon Web Services DMS Fleet Advisor end of support</a>.</p>
/// </important>
/// <p>Deletes the specified Fleet Advisor collector databases.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteFleetAdvisorDatabasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_fleet_advisor_databases::builders::DeleteFleetAdvisorDatabasesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesOutput,
        crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesError,
    > for DeleteFleetAdvisorDatabasesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesOutput,
            crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteFleetAdvisorDatabasesFluentBuilder {
    /// Creates a new `DeleteFleetAdvisorDatabasesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteFleetAdvisorDatabases as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_fleet_advisor_databases::builders::DeleteFleetAdvisorDatabasesInputBuilder {
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
        crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabases::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabases::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesOutput,
        crate::operation::delete_fleet_advisor_databases::DeleteFleetAdvisorDatabasesError,
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
    ///
    /// Appends an item to `DatabaseIds`.
    ///
    /// To override the contents of this collection use [`set_database_ids`](Self::set_database_ids).
    ///
    /// <p>The IDs of the Fleet Advisor collector databases to delete.</p>
    pub fn database_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_ids(input.into());
        self
    }
    /// <p>The IDs of the Fleet Advisor collector databases to delete.</p>
    pub fn set_database_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_database_ids(input);
        self
    }
    /// <p>The IDs of the Fleet Advisor collector databases to delete.</p>
    pub fn get_database_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_database_ids()
    }
}
