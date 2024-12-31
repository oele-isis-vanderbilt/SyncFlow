use std::time::Duration;

use rusoto_core::{credential::StaticProvider, region::Region};
use rusoto_credential::ProvideAwsCredentials;
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::GetObjectRequest;
use shared::deployment_config::S3Config;

pub struct StorageService {
    s3_config: S3Config,
    provider: StaticProvider,
    region: Region,
}

impl StorageService {
    pub fn new(config: &S3Config) -> Self {
        let credentials = StaticProvider::new_minimal(
            config.access_key.to_string(),
            config.secret_key.to_string(),
        );

        let region = Region::Custom {
            name: config.region.to_string(),
            endpoint: config.endpoint.to_string(),
        };

        StorageService {
            s3_config: config.clone(),
            provider: credentials,
            region,
        }
    }

    pub async fn generate_presigned_url(
        &self,
        path: &str,
        expires_in: Option<u64>,
    ) -> Result<String, rusoto_credential::CredentialsError> {
        let request = GetObjectRequest {
            bucket: self.s3_config.bucket.to_string(),
            key: path.to_string(),
            response_content_disposition: Some("attachment".to_string()),
            ..Default::default()
        };

        let s3_credentials = self.provider.credentials().await?;

        let option = PreSignedRequestOption {
            expires_in: Duration::from_secs(expires_in.unwrap_or(300)),
        };

        let url = request.get_presigned_url(&self.region, &s3_credentials, &option);

        Ok(url)
    }
}
