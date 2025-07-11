// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutInstanceCommunicationLimits`](crate::operation::put_instance_communication_limits::builders::PutInstanceCommunicationLimitsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connect_instance_id(impl Into<String>)`](crate::operation::put_instance_communication_limits::builders::PutInstanceCommunicationLimitsFluentBuilder::connect_instance_id) / [`set_connect_instance_id(Option<String>)`](crate::operation::put_instance_communication_limits::builders::PutInstanceCommunicationLimitsFluentBuilder::set_connect_instance_id):<br>required: **true**<br>Amazon Connect Instance Id<br>
    ///   - [`communication_limits_config(InstanceCommunicationLimitsConfig)`](crate::operation::put_instance_communication_limits::builders::PutInstanceCommunicationLimitsFluentBuilder::communication_limits_config) / [`set_communication_limits_config(Option<InstanceCommunicationLimitsConfig>)`](crate::operation::put_instance_communication_limits::builders::PutInstanceCommunicationLimitsFluentBuilder::set_communication_limits_config):<br>required: **true**<br>Instance Communication limits config<br>
    /// - On success, responds with [`PutInstanceCommunicationLimitsOutput`](crate::operation::put_instance_communication_limits::PutInstanceCommunicationLimitsOutput)
    /// - On failure, responds with [`SdkError<PutInstanceCommunicationLimitsError>`](crate::operation::put_instance_communication_limits::PutInstanceCommunicationLimitsError)
    pub fn put_instance_communication_limits(
        &self,
    ) -> crate::operation::put_instance_communication_limits::builders::PutInstanceCommunicationLimitsFluentBuilder {
        crate::operation::put_instance_communication_limits::builders::PutInstanceCommunicationLimitsFluentBuilder::new(self.handle.clone())
    }
}
