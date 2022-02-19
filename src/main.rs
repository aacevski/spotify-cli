extern crate dotenv;

use reqwest::Error;
use serde::Deserialize;
use std::env;
use url::Url;

mod lib;

#[derive(Deserialize, Debug)]
struct User {
    display_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let client = reqwest::Client::new();

    let authorization_url = lib::util::generate_authorization_url(
        env::var("SPOTIFY_ID").unwrap(),
        "http://localhost:3000/callback".to_string(),
    );
    // Opens the authorization URL in a new browser window.
    open::that(authorization_url).unwrap();

    let mut callback_url = String::new();
    println!("Enter the callback URL from the browser.");
    std::io::stdin().read_line(&mut callback_url).unwrap();

    let access_token = lib::util::create_spotify_client(Url::parse(&callback_url).unwrap()).await;

    let res = client
        .get("https://api.spotify.com/v1/me")
        .bearer_auth(access_token)
        .send()
        .await?;

    let user: User = res.json().await?;

    println!("Hello, {}!", user.display_name);

    Ok(())
}
