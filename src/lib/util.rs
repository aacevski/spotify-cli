use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use url::Url;

use std::env;

#[derive(Deserialize, Debug)]
struct Token {
    access_token: String,
}

fn get_scopes() -> HashSet<String> {
    return [
        "user-read-playback-state",
        "user-modify-playback-state",
        "user-read-currently-playing",
        "user-read-private",
        "user-read-email",
        "user-follow-modify",
        "user-follow-read",
        "user-library-modify",
        "user-library-read",
        "streaming",
        "user-read-playback-position",
        "user-top-read",
        "user-read-recently-played",
        "playlist-modify-private",
        "playlist-modify-public",
        "playlist-read-collaborative",
        "playlist-read-private",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<HashSet<String>>();
}

fn join_scopes() -> String {
    return get_scopes()
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
}

pub fn generate_authorization_url(client_id: String, redirect_uri: String) -> String {
    let scopes = join_scopes();

    let mut url_params: HashMap<&str, &str> = HashMap::new();
    url_params.insert("client_id", &client_id);
    url_params.insert("response_type", "code");
    url_params.insert("redirect_uri", &redirect_uri);
    url_params.insert("scope", &scopes);

    return Url::parse_with_params("https://accounts.spotify.com/authorize", &url_params)
        .unwrap()
        .to_string();
}

pub async fn create_spotify_client(callback_url: Url) -> String {
    let client = reqwest::Client::new();

    let code = callback_url
        .query_pairs()
        .find(|(key, _)| key == "code")
        .unwrap()
        .1
        .to_string();

    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("grant_type", "authorization_code");
    headers.insert("code", &code);
    headers.insert("redirect_uri", "http://localhost:3000/callback");

    let access_token = format!(
        "{}:{}",
        env::var("SPOTIFY_ID").unwrap(),
        env::var("SPOTIFY_SECRET").unwrap()
    );

    let access_token = format!("Basic {}", base64::encode(access_token));

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", access_token)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&headers)
        .send()
        .await;

    let token: Token = response.unwrap().json().await.unwrap();

    return token.access_token;
}
