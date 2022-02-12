use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

#[tokio::main]
async fn main() {
    let creds = Credentials::new(
        "27759326a5b8493a87ee6bcae5aae99a",
        "371582aeb9374bf3aa6c9090630dce01",
    );

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
