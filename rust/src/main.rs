use hyper::client::HttpConnector;
use rusoto_core::credential::StaticProvider;
use rusoto_core::{ByteStream, HttpClient, Region};
use rusoto_s3::{CreateBucketRequest, PutObjectRequest, S3Client, S3};
use std::env;
use tokio::runtime::Runtime;

const BUCKET_NAME: &str = "rust_test_bucket";
const FILE_NAME: &str = "rust_test_file";
const MESSAGE: &str = "You know what they say - a doctor a day keeps the apples away";

fn main() {
    let client = new_skynet_s3_client();
    let mut rt = Runtime::new().unwrap();

    // Create the bucket
    rt.block_on(create_bucket(&client));

    // Create key/file for the bucket
    rt.block_on(send_message_to_bucket(&client));
}

async fn create_bucket(client: &S3Client) {
    let mut request = CreateBucketRequest::default();

    request.bucket = BUCKET_NAME.to_string();
    client
        .create_bucket(request)
        .await
        .expect("Failed to create test bucket");
}

async fn send_message_to_bucket(client: &S3Client) {
    let mut request = PutObjectRequest::default();

    request.bucket = BUCKET_NAME.to_string();
    request.key = FILE_NAME.to_string();
    request.body = Some(ByteStream::from(MESSAGE.as_bytes().to_vec()));
    client
        .put_object(request)
        .await
        .expect("Failed to sent message to bucket");
}

fn new_skynet_s3_client() -> S3Client {
    let (access_key_id, secret_access_key, skynet_server_endpoint) = load_env();

    // Create http connector - make sure to not enforce http
    let mut http_connector = HttpConnector::new();
    http_connector.enforce_http(false);

    // Create rusoto http client to be used in rusoto services
    let http_client = HttpClient::from_connector(http_connector);

    // Build some credentials
    let cred_provider = StaticProvider::new(access_key_id, secret_access_key, None, None);

    // Custom region to talk to local service
    let local_region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: skynet_server_endpoint,
    };

    // Initialize s3 client with our configuration
    return S3Client::new_with(http_client, cred_provider, local_region);
}

fn load_env() -> (String, String, String) {
    let mut env_vars: (String, String, String) = ("".to_string(), "".to_string(), "".to_string());
    match env::var("ACCESS_KEY_ID") {
        Ok(var) => env_vars.0 = var,
        Err(e) => panic!("{}: ACCESS_KEY_ID", e),
    };
    match env::var("SECRET_KEY") {
        Ok(var) => env_vars.1 = var,
        Err(e) => panic!("{}: SECRET_KEY", e),
    };
    match env::var("SKYNET_S3_SERVER") {
        Ok(var) => env_vars.2 = var,
        Err(e) => panic!("{}: SKYNET_S3_SERVER", e),
    };
    env_vars
}
