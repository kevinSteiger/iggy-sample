use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client = IggyClient::default();
    client.connect().await?;


    Ok(()) 
}