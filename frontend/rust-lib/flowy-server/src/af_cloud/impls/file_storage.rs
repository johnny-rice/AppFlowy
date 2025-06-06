use crate::af_cloud::AFServer;
use client_api::entity::{CompleteUploadRequest, CreateUploadRequest};
use flowy_error::{ErrorCode, FlowyError, FlowyResult};
use flowy_storage_pub::cloud::{ObjectIdentity, ObjectValue, StorageCloudService};
use flowy_storage_pub::storage::{CompletedPartRequest, CreateUploadResponse, UploadPartResponse};
use lib_infra::async_trait::async_trait;
use uuid::Uuid;

pub struct AFCloudFileStorageServiceImpl<T> {
  pub client: T,

  /// Only use in debug mode
  pub maximum_upload_file_size_in_bytes: Option<u64>,
}

impl<T> AFCloudFileStorageServiceImpl<T> {
  pub fn new(client: T, maximum_upload_file_size_in_bytes: Option<u64>) -> Self {
    Self {
      client,
      maximum_upload_file_size_in_bytes,
    }
  }
}

#[async_trait]
impl<T> StorageCloudService for AFCloudFileStorageServiceImpl<T>
where
  T: AFServer,
{
  async fn get_object_url(&self, object_id: ObjectIdentity) -> Result<String, FlowyError> {
    let file_name = format!("{}.{}", object_id.file_id, object_id.ext);
    let url = self
      .client
      .try_get_client()?
      .get_blob_url(&object_id.workspace_id, &file_name);
    Ok(url)
  }

  async fn put_object(&self, url: String, file: ObjectValue) -> Result<(), FlowyError> {
    let client = self.client.try_get_client()?;
    client.put_blob(&url, file.raw, &file.mime).await?;
    Ok(())
  }

  async fn delete_object(&self, url: &str) -> Result<(), FlowyError> {
    self.client.try_get_client()?.delete_blob(url).await?;
    Ok(())
  }

  async fn get_object(&self, url: String) -> Result<ObjectValue, FlowyError> {
    let (mime, raw) = self.client.try_get_client()?.get_blob(&url).await?;
    Ok(ObjectValue {
      raw: raw.into(),
      mime,
    })
  }

  async fn get_object_url_v1(
    &self,
    workspace_id: &Uuid,
    parent_dir: &str,
    file_id: &str,
  ) -> FlowyResult<String> {
    let url = self
      .client
      .try_get_client()?
      .get_blob_url_v1(workspace_id, parent_dir, file_id);
    Ok(url)
  }

  async fn parse_object_url_v1(&self, url: &str) -> Option<(Uuid, String, String)> {
    let value = self.client.try_get_client().ok()?.parse_blob_url_v1(url)?;
    Some(value)
  }

  async fn create_upload(
    &self,
    workspace_id: &Uuid,
    parent_dir: &str,
    file_id: &str,
    content_type: &str,
    file_size: u64,
  ) -> Result<CreateUploadResponse, FlowyError> {
    let parent_dir = parent_dir.to_string();
    let content_type = content_type.to_string();
    let file_id = file_id.to_string();
    let try_get_client = self.client.try_get_client();
    let client = try_get_client?;
    let req = CreateUploadRequest {
      file_id,
      parent_dir,
      content_type,
      file_size: Some(file_size),
    };

    if cfg!(debug_assertions) {
      if let Some(maximum_upload_size) = self.maximum_upload_file_size_in_bytes {
        if file_size > maximum_upload_size {
          return Err(FlowyError::new(
            ErrorCode::SingleUploadLimitExceeded,
            "File size exceeds the maximum limit",
          ));
        }
      }
    }

    let resp = client.create_upload(workspace_id, req).await?;
    Ok(resp)
  }

  async fn upload_part(
    &self,
    workspace_id: &Uuid,
    parent_dir: &str,
    upload_id: &str,
    file_id: &str,
    part_number: i32,
    body: Vec<u8>,
  ) -> Result<UploadPartResponse, FlowyError> {
    let try_get_client = self.client.try_get_client();
    let client = try_get_client?;
    let resp = client
      .upload_part(
        workspace_id,
        parent_dir,
        file_id,
        upload_id,
        part_number,
        body,
      )
      .await?;

    Ok(resp)
  }

  async fn complete_upload(
    &self,
    workspace_id: &Uuid,
    parent_dir: &str,
    upload_id: &str,
    file_id: &str,
    parts: Vec<CompletedPartRequest>,
  ) -> Result<(), FlowyError> {
    let parent_dir = parent_dir.to_string();
    let upload_id = upload_id.to_string();
    let file_id = file_id.to_string();
    let try_get_client = self.client.try_get_client();
    let client = try_get_client?;
    let request = CompleteUploadRequest {
      file_id,
      parent_dir,
      upload_id,
      parts,
    };
    client.complete_upload(workspace_id, request).await?;
    Ok(())
  }
}
