use reqwest::Error;
use reqwest::Response;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    username: String,
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://jsonplaceholder.typicode.com/users");
    println!("{}", request_url);
    let response: Response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json::<Vec<User>>().await?;
    for user in users {
        println!("{:?}", user);
    }
    Ok(())
}
