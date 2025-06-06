use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use client_api::entity::AFRole;
pub use client_api::entity::billing_dto::RecurringInterval;
use flowy_error::FlowyResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::*;
use uuid::Uuid;

pub const USER_METADATA_ICON_URL: &str = "icon_url";
pub const USER_METADATA_UPDATE_AT: &str = "updated_at";

pub trait UserAuthResponse {
  fn user_id(&self) -> i64;
  fn user_uuid(&self) -> &Uuid;
  fn user_name(&self) -> &str;
  fn latest_workspace(&self) -> &UserWorkspace;
  fn user_workspaces(&self) -> &[UserWorkspace];
  fn user_token(&self) -> Option<String>;
  fn user_email(&self) -> Option<String>;
  fn encryption_type(&self) -> EncryptionType;
  fn metadata(&self) -> &Option<serde_json::Value>;
  fn updated_at(&self) -> i64;
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SignInParams {
  pub email: String,
  pub password: String,
  pub name: String,
  pub auth_type: AuthType,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SignUpParams {
  pub email: String,
  pub name: String,
  pub password: String,
  pub auth_type: AuthType,
  pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthResponse {
  pub user_id: i64,
  pub user_uuid: Uuid,
  pub name: String,
  pub latest_workspace: UserWorkspace,
  pub user_workspaces: Vec<UserWorkspace>,
  pub is_new_user: bool,
  pub email: Option<String>,
  pub token: Option<String>,
  pub encryption_type: EncryptionType,
  pub updated_at: i64,
  pub metadata: Option<serde_json::Value>,
}

impl UserAuthResponse for AuthResponse {
  fn user_id(&self) -> i64 {
    self.user_id
  }

  fn user_uuid(&self) -> &Uuid {
    &self.user_uuid
  }

  fn user_name(&self) -> &str {
    &self.name
  }

  fn latest_workspace(&self) -> &UserWorkspace {
    &self.latest_workspace
  }

  fn user_workspaces(&self) -> &[UserWorkspace] {
    &self.user_workspaces
  }

  fn user_token(&self) -> Option<String> {
    self.token.clone()
  }

  fn user_email(&self) -> Option<String> {
    self.email.clone()
  }

  fn encryption_type(&self) -> EncryptionType {
    self.encryption_type.clone()
  }

  fn metadata(&self) -> &Option<Value> {
    &self.metadata
  }

  fn updated_at(&self) -> i64 {
    self.updated_at
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserWorkspace {
  pub id: String,
  pub name: String,
  pub created_at: DateTime<Utc>,
  /// The database storage id is used indexing all the database views in current workspace.
  #[serde(rename = "database_storage_id")]
  pub workspace_database_id: String,
  #[serde(default)]
  pub icon: String,
  #[serde(default)]
  pub member_count: i64,
  #[serde(default)]
  pub role: Option<Role>,
  #[serde(default = "default_workspace_type")]
  pub workspace_type: WorkspaceType,
}

fn default_workspace_type() -> WorkspaceType {
  WorkspaceType::Server
}

impl UserWorkspace {
  pub fn workspace_id(&self) -> FlowyResult<Uuid> {
    let id = Uuid::from_str(&self.id)?;
    Ok(id)
  }

  pub fn new_local(workspace_id: String, name: &str) -> Self {
    Self {
      id: workspace_id,
      name: name.to_string(),
      created_at: Utc::now(),
      workspace_database_id: Uuid::new_v4().to_string(),
      icon: "".to_string(),
      member_count: 1,
      role: Some(Role::Owner),
      workspace_type: WorkspaceType::Local,
    }
  }
}

#[derive(Default, Debug, Clone)]
pub struct UserProfile {
  pub uid: i64,
  pub email: String,
  pub name: String,
  pub token: String,
  pub icon_url: String,
  pub auth_type: AuthType,
  pub workspace_type: WorkspaceType,
  pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub enum EncryptionType {
  #[default]
  NoEncryption,
  SelfEncryption(String),
}

impl EncryptionType {
  pub fn from_sign(sign: &str) -> Self {
    if sign.is_empty() {
      EncryptionType::NoEncryption
    } else {
      EncryptionType::SelfEncryption(sign.to_owned())
    }
  }

  pub fn require_encrypt_secret(&self) -> bool {
    match self {
      EncryptionType::NoEncryption => false,
      EncryptionType::SelfEncryption(sign) => !sign.is_empty(),
    }
  }

  pub fn sign(&self) -> String {
    match self {
      EncryptionType::NoEncryption => "".to_owned(),
      EncryptionType::SelfEncryption(sign) => sign.to_owned(),
    }
  }
}

impl FromStr for EncryptionType {
  type Err = serde_json::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    serde_json::from_str(s)
  }
}

impl<T> From<(&T, &AuthType)> for UserProfile
where
  T: UserAuthResponse,
{
  fn from(params: (&T, &AuthType)) -> Self {
    let (value, auth_type) = params;
    let icon_url = value
      .metadata()
      .as_ref()
      .map(|m| {
        m.get(USER_METADATA_ICON_URL)
          .map(|v| v.as_str().map(|s| s.to_string()).unwrap_or_default())
          .unwrap_or_default()
      })
      .unwrap_or_default();
    let workspace_type = WorkspaceType::from(auth_type);
    Self {
      uid: value.user_id(),
      email: value.user_email().unwrap_or_default(),
      name: value.user_name().to_owned(),
      token: value.user_token().unwrap_or_default(),
      icon_url,
      auth_type: *auth_type,
      workspace_type,
      updated_at: value.updated_at(),
    }
  }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct UpdateUserProfileParams {
  pub uid: i64,
  pub name: Option<String>,
  pub email: Option<String>,
  pub password: Option<String>,
  pub icon_url: Option<String>,
  pub token: Option<String>,
}

impl UpdateUserProfileParams {
  pub fn new(uid: i64) -> Self {
    Self {
      uid,
      ..Default::default()
    }
  }

  pub fn with_token(mut self, token: String) -> Self {
    self.token = Some(token);
    self
  }

  pub fn with_name<T: ToString>(mut self, name: T) -> Self {
    self.name = Some(name.to_string());
    self
  }

  pub fn with_email<T: ToString>(mut self, email: T) -> Self {
    self.email = Some(email.to_string());
    self
  }

  pub fn with_password<T: ToString>(mut self, password: T) -> Self {
    self.password = Some(password.to_string());
    self
  }

  pub fn with_icon_url<T: ToString>(mut self, icon_url: T) -> Self {
    self.icon_url = Some(icon_url.to_string());
    self
  }
}

#[derive(Debug, Clone, Copy, Hash, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum WorkspaceType {
  Local = 0,
  Server = 1,
}

impl Default for WorkspaceType {
  fn default() -> Self {
    Self::Local
  }
}

impl WorkspaceType {
  pub fn is_local(&self) -> bool {
    matches!(self, WorkspaceType::Local)
  }
}

impl From<i32> for WorkspaceType {
  fn from(value: i32) -> Self {
    match value {
      0 => WorkspaceType::Local,
      1 => WorkspaceType::Server,
      _ => WorkspaceType::Server,
    }
  }
}

impl From<&AuthType> for WorkspaceType {
  fn from(value: &AuthType) -> Self {
    match value {
      AuthType::Local => WorkspaceType::Local,
      AuthType::AppFlowyCloud => WorkspaceType::Server,
    }
  }
}

#[derive(Debug, Clone, Copy, Hash, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum AuthType {
  /// It's a local server, we do fake sign in default.
  Local = 0,
  /// Currently not supported. It will be supported in the future when the
  /// [AppFlowy-Server](https://github.com/AppFlowy-IO/AppFlowy-Server) ready.
  AppFlowyCloud = 1,
}

impl From<WorkspaceType> for AuthType {
  fn from(value: WorkspaceType) -> Self {
    match value {
      WorkspaceType::Local => AuthType::Local,
      WorkspaceType::Server => AuthType::AppFlowyCloud,
    }
  }
}

impl Display for AuthType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      AuthType::Local => write!(f, "Local"),
      AuthType::AppFlowyCloud => write!(f, "AppFlowyCloud"),
    }
  }
}

impl Default for AuthType {
  fn default() -> Self {
    Self::Local
  }
}

impl AuthType {
  pub fn is_local(&self) -> bool {
    matches!(self, AuthType::Local)
  }

  pub fn is_appflowy_cloud(&self) -> bool {
    matches!(self, AuthType::AppFlowyCloud)
  }
}

impl From<i32> for AuthType {
  fn from(value: i32) -> Self {
    match value {
      0 => AuthType::Local,
      1 => AuthType::AppFlowyCloud,
      _ => AuthType::Local,
    }
  }
}
pub struct SupabaseOAuthParams {
  pub uuid: Uuid,
  pub email: String,
}

pub struct AFCloudOAuthParams {
  pub sign_in_url: String,
}

#[derive(Clone, Debug)]
pub enum UserTokenState {
  Init,
  Refresh { token: String },
  Invalid,
}

// Workspace Role
#[derive(Clone, Copy, Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum Role {
  Owner = 0,
  Member = 1,
  Guest = 2,
}

impl From<i32> for Role {
  fn from(value: i32) -> Self {
    match value {
      0 => Role::Owner,
      1 => Role::Member,
      2 => Role::Guest,
      _ => Role::Guest,
    }
  }
}

impl From<Role> for i32 {
  fn from(value: Role) -> Self {
    match value {
      Role::Owner => 0,
      Role::Member => 1,
      Role::Guest => 2,
    }
  }
}

impl From<AFRole> for Role {
  fn from(value: AFRole) -> Self {
    match value {
      AFRole::Owner => Role::Owner,
      AFRole::Member => Role::Member,
      AFRole::Guest => Role::Guest,
    }
  }
}

pub struct WorkspaceMember {
  pub email: String,
  pub role: Role,
  pub name: String,
  pub avatar_url: Option<String>,
  pub joined_at: Option<i64>,
}

/// represent the user awareness object id for the workspace.
pub fn user_awareness_object_id(user_uuid: &Uuid, workspace_id: &str) -> Uuid {
  Uuid::new_v5(
    user_uuid,
    format!("user_awareness:{}", workspace_id).as_bytes(),
  )
}

#[derive(Clone, Debug)]
pub enum WorkspaceInvitationStatus {
  Pending,
  Accepted,
  Rejected,
}

pub struct WorkspaceInvitation {
  pub invite_id: Uuid,
  pub workspace_id: Uuid,
  pub workspace_name: Option<String>,
  pub inviter_email: Option<String>,
  pub inviter_name: Option<String>,
  pub status: WorkspaceInvitationStatus,
  pub updated_at: DateTime<Utc>,
}
