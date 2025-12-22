use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub bucket: Bucket,
}

impl AppState {
    pub async fn new_from_env() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")?;
        let jwt_secret = env::var("JWT_SECRET")?;
        
        // S3 Config
        let s3_endpoint = env::var("S3_ENDPOINT").unwrap_or("http://localhost:9000".to_string());
        let s3_access = env::var("S3_ACCESS_KEY").unwrap_or("minio".to_string());
        let s3_secret = env::var("S3_SECRET_KEY").unwrap_or("minio123".to_string());
        let s3_bucket_name = env::var("S3_BUCKET_NAME").unwrap_or("speakeasy-attachments".to_string());
        let s3_region_name = env::var("S3_REGION").unwrap_or("us-east-1".to_string());

        let region = Region::Custom { region: s3_region_name, endpoint: s3_endpoint };
        let credentials = Credentials::new(Some(&s3_access), Some(&s3_secret), None, None, None)?;
        
        let bucket = Bucket::new(&s3_bucket_name, region, credentials)?.with_path_style();

        let db = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;

        Ok(Self { db, jwt_secret, bucket })
    }
}
