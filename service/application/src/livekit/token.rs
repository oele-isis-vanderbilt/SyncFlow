use livekit_api::access_token;
use livekit_api::access_token::AccessTokenError;
use shared::deployment_config::DeploymentConfig;

use shared::livekit_models::TokenRequest;

pub fn create_token(req: &TokenRequest, config: &DeploymentConfig) -> Result<String, AccessTokenError> {
    let api_key = config.livekit_api_key.clone();
    let api_secret = config.livekit_api_secret.clone();

    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity(&req.identity)
        .with_name(&req.identity)
        .with_grants(req.video_grants.clone().into());

    token.to_jwt()
}
