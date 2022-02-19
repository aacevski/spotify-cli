use reqwest::Error;
use std::collections::HashMap;
use std::collections::HashSet;
use url::Url;
extern crate dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let client = reqwest::Client::new();

    let mut scopes: HashSet<String> = HashSet::new();

    scopes.insert("user-read-currently-playing".to_string());
    scopes.insert("user-read-recently-played".to_string());

    let scopes = join_scopes(&scopes);

    let url = generate_authorization_url(
        scopes,
        env::var("SPOTIFY_ID").unwrap(),
        "http://localhost:3000/callback".to_string(),
    );

    println!("{}", url);

    open::that(url).unwrap();

    let mut callback_url = String::new();
    println!("Enter the callback URL from the browser");
    std::io::stdin().read_line(&mut callback_url).unwrap();

    let callback_url = Url::parse(&callback_url).unwrap();

    let code = callback_url
        .query()
        .unwrap()
        .to_owned()
        .replace("code=", "");

    let mut map = HashMap::new();
    map.insert("grant_type", "authorization_code");
    map.insert("code", &code);
    map.insert("redirect_uri", "http://localhost:3000/callback");

    let value = format!(
        "{}:{}",
        env::var("SPOTIFY_ID").unwrap(),
        env::var("SPOTIFY_SECRET").unwrap()
    );
    let value = format!("Basic {}", base64::encode(value));

    let res = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", value)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&map)
        .send()
        .await?;

    println!("{:?}", res.text().await?);

    Ok(())
}

fn join_scopes(scopes: &HashSet<String>) -> String {
    return scopes
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
}

fn generate_authorization_url(scopes: String, client_id: String, redirect_uri: String) -> String {
    let mut url_params: HashMap<&str, &str> = HashMap::new();
    url_params.insert("client_id", &client_id);
    url_params.insert("response_type", "code");
    url_params.insert("redirect_uri", &redirect_uri);
    url_params.insert("scope", &scopes);

    let parsed_url = Url::parse_with_params("https://accounts.spotify.com/authorize", &url_params);

    return parsed_url.unwrap().to_string();
}
