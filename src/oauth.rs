use actix_web::{web, HttpResponse, Responder};
use dotenv::dotenv;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl,
};
use std::collections::HashMap;
use std::env;

pub async fn openai_login() -> impl Responder {
    dotenv().ok();
    let client_id = env::var("OPENAI_CLIENT_ID").expect("OPENAI_CLIENT_ID must be set");
    let client_secret = env::var("OPENAI_CLIENT_SECRET").expect("OPENAI_CLIENT_SECRET must be set");
    let redirect_uri = env::var("OPENAI_REDIRECT_URI").expect("OPENAI_REDIRECT_URI must be set");

    let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://api.openai.com/v1/oauth/authorize".to_string()).unwrap(),
        TokenUrl::new("https://api.openai.com/v1/oauth/token".to_string()).ok(),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

    // Generate a PKCE challenge.
    let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorize_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .url();

    HttpResponse::Found()
        .append_header(("Location", authorize_url.to_string()))
        .finish()
}

pub async fn openai_callback(_query: web::Query<HashMap<String, String>>) -> impl Responder {
    // Here you would extract the code from the query parameters, exchange it for an access token,
    // and then use that token to make API requests on behalf of the user.
    // This is a placeholder to indicate where you would implement this logic.
    HttpResponse::Ok().body("Callback endpoint - implement token exchange here")
}
