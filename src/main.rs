use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, InputLogEvent, PutLogEventsRequest,
};
use rusoto_core::Region;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Arc::new(CloudWatchLogsClient::new(Region::default()));

    // Define multiple log groups and streams
    let log_targets = vec![
        ("tiopaco", "tiootoo"),
        ("tiopaco", "mylogstream"),
    ];

    // Vec to hold the handles of spawned tasks
    let mut handles = Vec::new();

    for (log_group, log_stream) in log_targets {
        let client = client.clone();
        let handle = task::spawn(async move {
            let sequence_token = None; // Replace with actual sequence token if necessary

            let mut events = Vec::new();
            // Create log events
            for i in 0..10000 {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_millis() as i64; // CloudWatch expects the timestamp in milliseconds

                // Creating a JSON object with i and timestamp
                let inner_message = json!({
                    "value": i,
                    "timestamp": timestamp
                });

                // Serializing the inner JSON object to a string
                let message = inner_message.to_string();

                let event = InputLogEvent {
                    message,
                    timestamp,
                };

                events.push(event);
            }

            let put_log_events = PutLogEventsRequest {
                log_events: events,
                log_group_name: log_group.to_string(),
                log_stream_name: log_stream.to_string(),
                sequence_token,
            };

            match client.put_log_events(put_log_events).await {
                Ok(output) => {
                    println!("PutLogEvents succeeded for {} {}. Next sequence token: {:?}", log_group, log_stream, output.next_sequence_token);
                }
                Err(e) => {
                    eprintln!("PutLogEvents error for {} {}: {:?}", log_group, log_stream, e);
                }
            }
        });

        handles.push(handle);
    }

    // Await on all the handles
    for handle in handles {
        handle.await.unwrap();
    }
}
