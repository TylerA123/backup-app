// Backblaze B2 client wrapper
// Uses aws-sdk-s3 with B2-compatible endpoint

use aws_sdk_s3::config::{Region, Credentials};
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use std::path::Path;
use tokio_stream::StreamExt;
use tracing::info;

pub struct B2Client {
    client: Client,
    bucket: String,
}

impl B2Client {
    pub fn new(key_id: &str, app_key: &str, bucket: &str, endpoint: &str) -> Self {
        let creds = Credentials::new(key_id, app_key, None, None, "b2");
        let config = aws_sdk_s3::Config::builder()
            .region(Region::new("us-west-002"))
            .endpoint_url(endpoint)
            .credentials_provider(creds)
            .force_path_style(true)
            .build();

        B2Client {
            client: Client::from_conf(config),
            bucket: bucket.to_string(),
        }
    }

    pub async fn upload_file(
        &self,
        key: &str,
        file_path: &Path,
    ) -> Result<String, String> {
        let body = ByteStream::from_path(file_path)
            .await
            .map_err(|e| format!("Cannot read file: {}", e))?;

        let result = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await
            .map_err(|e| format!("Upload failed: {}", e))?;

        let etag = result.e_tag().unwrap_or("unknown");
        info!("Uploaded {} (etag: {})", key, etag);
        Ok(etag.to_string())
    }

    pub async fn download_file(
        &self,
        key: &str,
        destination: &Path,
    ) -> Result<(), String> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| format!("Download failed: {}", e))?;

        let mut body = result.body;
        let mut file = std::fs::File::create(destination)
            .map_err(|e| format!("Cannot create file: {}", e))?;

        use std::io::Write;
        while let Some(chunk) = body
            .try_next()
            .await
            .map_err(|e| format!("Stream error: {}", e))?
        {
            file.write_all(&chunk)
                .map_err(|e| format!("Write error: {}", e))?;
        }

        info!("Downloaded {} to {:?}", key, destination);
        Ok(())
    }

    pub async fn check_object_exists(&self, key: &str) -> Result<bool, String> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("404") {
                    Ok(false)
                } else {
                    Err(format!("Head object failed: {}", e))
                }
            }
        }
    }
}
