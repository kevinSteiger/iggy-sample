use iggy::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = IggyClient::from_connection_string("iggy://iggy:iggy@127.0.0.1:8090")?;
    client.connect().await?;
    let stream = Identifier::named("sample-stream")?;
    let topic = Identifier::named("sample-topic")?;

    match client.create_stream("sample-stream", &StreamCreateOptions::default()).await {
        Ok(_) | Err(IggyError::StreamNameAlreadyExists(_)) => {}
        Err(error) => return Err(error.into()),
    }

    match client.create_topic(&stream, "sample-topic", &TopicCreateOptions {
        partitions_count: Some(1),
        message_expiry: Some(IggyExpiry::NeverExpire),
        ..TopicCreateOptions::default()
    }).await {
        Ok(_) | Err(IggyError::TopicNameAlreadyExists(_, _)) => {}
        Err(error) => return Err(error.into()),
    }



    let mut messages = (1..=10)
    .map(|i| IggyMessage::from_str(&format!("message-{i}")))
    .collect::<Result<Vec<_>, _>>()?;

    let response = client.send_messages(
        &stream, &topic, &Partitioning::partition_id(0), &mut messages,
    ).await?;
    for confirmation in response.confirmations {
        println!("Partition {}, base offset {}", confirmation.partition_id, confirmation.base_offset);
    }
    client.shutdown().await?;

    Ok(()) 
}