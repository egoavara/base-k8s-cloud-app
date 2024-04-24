use futures::{future, FutureExt, StreamExt};
use std::sync::Arc;
use temporal_client::{ClientOptionsBuilder, WorkflowClientTrait, WorkflowService};
use temporal_sdk::{WfContext, Worker, WorkflowResult};
use temporal_sdk_core::api::telemetry::{TelemetryOptions, TelemetryOptionsBuilder};
use temporal_sdk_core::protos::temporal::api::common::v1::{Payloads, WorkflowType};
use temporal_sdk_core::protos::temporal::api::enums::v1::TaskQueueKind;
use temporal_sdk_core::protos::temporal::api::taskqueue::v1::TaskQueue;
use temporal_sdk_core::protos::temporal::api::workflowservice::v1::StartWorkflowExecutionRequest;
use temporal_sdk_core::{CoreRuntime, WorkerConfigBuilder};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let temporal_url = temporal_sdk_core::Url::parse("http://temporal-frontend-headless.event-system.svc:7233").unwrap();
    let client_option = ClientOptionsBuilder::default()
        .identity("event-otype".to_string())
        .target_url(temporal_url)
        .client_name("event-otype".to_string())
        .client_version("0.1.0".to_string())
        .build()
        .unwrap();
    let mut client = Arc::new(client_option.connect("default", None).await.unwrap());
    client.signal_workflow_execution()
    client.start_workflow_execution(StartWorkflowExecutionRequest {
        namespace: "default".to_string(),
        input: Some(Payloads { payloads: vec![] }),
        workflow_id: ,
        workflow_type: Some(WorkflowType { name: workflow_type }),
        task_queue: Some(TaskQueue {
            name: task_queue,
            kind: TaskQueueKind::Unspecified as i32,
            normal_name: "".to_string(),
        }),
        request_id: request_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
        workflow_id_reuse_policy: options.id_reuse_policy as i32,
        workflow_execution_timeout: options.execution_timeout.and_then(|d| d.try_into().ok()),
        workflow_run_timeout: options.run_timeout.and_then(|d| d.try_into().ok()),
        workflow_task_timeout: options.task_timeout.and_then(|d| d.try_into().ok()),
        search_attributes: options.search_attributes.map(|d| d.into()),
        cron_schedule: options.cron_schedule.unwrap_or_default(),
        request_eager_execution: options.enable_eager_workflow_start,
        retry_policy: options.retry_policy,
        ..Default::default()
    })
        .await
        .unwrap();
}
