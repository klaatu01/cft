use rusoto_cloudformation::{
    CloudFormation, CloudFormationClient, ListStackResourcesInput, StackResourceSummary,
};
use rusoto_core::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, DescribeLogStreamsRequest, GetLogEventsRequest,
    OutputLogEvent,
};

pub async fn describe_stack_resources(stack_name: String) -> Option<Vec<StackResourceSummary>> {
    let client = CloudFormationClient::new(Region::default());
    let input = ListStackResourcesInput {
        next_token: None,
        stack_name,
    };
    let output = client.list_stack_resources(input).await;
    match output {
        Ok(r) => r.stack_resource_summaries,
        _ => None,
    }
}

pub async fn get_latest_logs(log_group_name: String, number: usize) -> Option<Vec<OutputLogEvent>> {
    let client = CloudWatchLogsClient::new(Region::default());
    let log_stream_request = DescribeLogStreamsRequest {
        log_group_name: log_group_name.clone(),
        limit: Some(1),
        descending: Some(true),
        ..Default::default()
    };

    let stream_response = client
        .describe_log_streams(log_stream_request)
        .await
        .unwrap();

    let log_stream_name = stream_response
        .log_streams
        .unwrap()
        .get(0)
        .unwrap()
        .log_stream_name
        .clone()
        .unwrap();

    let log_events_request = GetLogEventsRequest {
        log_group_name,
        log_stream_name,
        limit: Some(10),
        ..Default::default()
    };

    client
        .get_log_events(log_events_request)
        .await
        .unwrap()
        .events
}
