use azure_core::auth::TokenCredential;
use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use log::debug;
use polars::frame::DataFrame;

use crate::data_service::read_csv_from_string;
use crate::entities::APIError;

pub async fn fetch_dataset_az_blob(
    account_name: &str,
    container_name: &str,
    blob_name: &str,
) -> Result<DataFrame, APIError> {
    debug!(
        "Fetching dataset from Azure Blob Storage: {}/{}/{}",
        account_name, container_name, blob_name
    );

    let credential = DefaultAzureCredential::create(TokenCredentialOptions::default())
        .map_err(|e| APIError::new(&format!("Failed to create Azure credential: {}", e)))?;

    let access_token = credential
        .get_token(&["https://storage.azure.com/.default"])
        .await
        .map_err(|e| APIError::new(&format!("Failed to get Azure token: {}", e)))?;

    // Create StorageCredentials using the token
    let storage_credentials =
        StorageCredentials::bearer_token(access_token.token.secret().to_string());

    // Create a blob service client using the StorageCredentials
    let blob_service_client = BlobServiceClient::new(account_name, storage_credentials);
    let container_client = blob_service_client.container_client(container_name);

    // Get a blob client
    let blob_client = container_client.blob_client(blob_name);

    // Read the blob in chunks
    let mut stream = blob_client.get().into_stream(); // Read in 1 MB chunks
    let mut buffer = Vec::new();

    while let Some(value) = stream.next().await {
        match value {
            Ok(bytes) => {
                // Process the chunk of bytes
                // For example, you can write to a file or process the data
                let datas = bytes.data.collect().await;
                match datas {
                    Ok(data) => {
                        let data = String::from_utf8_lossy(&data);
                        buffer.push(data.to_string());
                    }
                    Err(e) => {
                        return Err(APIError::new(&format!("Error reading blob: {}", e)));
                    }
                }
            }
            Err(e) => {
                return Err(APIError::new(&format!("Error reading blob: {}", e)));
            }
        }
    }
    let csv_data = buffer.join("\n");
    debug!("CSV data: {}", csv_data);

    let dataset = read_csv_from_string(&csv_data)
        .map_err(|e| APIError::new(&format!("Failed to read CSV from string: {}", e)))?;

    Ok(dataset)
}
