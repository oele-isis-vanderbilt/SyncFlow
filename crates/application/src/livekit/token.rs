use livekit_api::access_token;
use livekit_api::access_token::AccessTokenError;

use shared::livekit_models::TokenRequest;

pub fn create_token(
    req: &TokenRequest,
    api_key: &str,
    api_secret: &str,
) -> Result<String, AccessTokenError> {
    let name = req.name.as_deref().unwrap_or(req.identity.as_str());
    let identity = req.identity.as_str();
    let token = access_token::AccessToken::with_api_key(api_key, api_secret)
        .with_identity(identity)
        .with_name(name)
        .with_grants(req.video_grants.clone().into());

    token.to_jwt()
}
