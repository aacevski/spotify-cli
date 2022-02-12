use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

#[tokio::main]
async fn main() {
    let creds = Credentials::new();

    let oauth = OAuth {
        redirect_uri: "http://localhost:3000/callback".to_string(),
        scopes: scopes!("user-read-recently-played"),
        ..Default::default()
    };

    let mut spotify = AuthCodeSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let history = spotify.current_playback(None, None::<Vec<_>>).await;

    println!("Response: {:?}", history);
}
