use livekit_api::access_token;
use livekit_api::access_token::AccessTokenError;
use std::env;

use crate::models::TokenRequest;

pub fn create_token(req: &TokenRequest) -> Result<String, AccessTokenError> {
    let api_key = env::var("LIVEKIT_API_KEY")?;
    let api_secret = env::var("LIVEKIT_API_SECRET")?;

    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity(&req.identity)
        .with_name(&req.identity)
        .with_grants(req.video_grants.clone().into());

    token.to_jwt()
}
