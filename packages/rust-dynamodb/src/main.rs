use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::first_try("us-west-2").or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    dbg!(&shared_config);
    let client = Client::new(&shared_config);
    let res = client.list_tables().send().await?;
    dbg!(res.table_names);
    Ok(())
}
