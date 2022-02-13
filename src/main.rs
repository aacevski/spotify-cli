use reqwest::header::HeaderMap;
use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: u32,
    scope: String,
}

#[derive(Deserialize, Debug)]
struct TopArtists {
    timestamp: usize,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let value = format!(
        "{}:{}",
        "140526a5c3074a7cba86129838ea667b", "f819e6031f2b4ab58e6382212eede196"
    );
    let value = format!("Basic {}", base64::encode(value));

    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("grant_type", "refresh_token");
    map.insert("refresh_token", "AQAfSpxhdj5URPVooNoUN8RxDVPOKIcZLgkPNfwkJ7V8yCremIt05sW2OMgVec7l2Cft3uln6gvRNoNLsC9kQcg__fbr81GgmOhQ2i5OhjFYsKzk1eIwZ8Q8T85CRciII_4");

    let res = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", value)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&map)
        .send()
        .await?;

    let value: AccessToken = res.json().await?;

    let response = client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .header("Authorization", format!("Bearer {}", value.access_token))
        .send()
        .await?;

    let artists: TopArtists = response.json().await?;

    println!("Response: {:?}", artists);

    Ok(())
}
