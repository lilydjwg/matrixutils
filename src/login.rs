use std::path::Path;
use std::io::Write;

use matrix_sdk::ruma;
use matrix_sdk::{
  Client,
  authentication::matrix::MatrixSession,
  authentication::SessionTokens,
  SessionMeta,
};
use tracing::info;
use eyre::Result;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct LoginInfo {
  homeserver: url::Url,
  user_id: String,
  device_id: String,
  access_token: String,
}

pub async fn sso_login<P: AsRef<Path>>(
  loginfile: P,
  server_name: &ruma::ServerName,
) -> Result<Client> {
  info!("sso login.");
  let client = build_client(Some(server_name), None).await?;

  let auth = client.matrix_auth();
  let url = auth.get_sso_login_url("http://localhost:8088/login", None).await?;
  println!("Visit {} to login.", url);

  let mut token = String::new();
  std::io::stdin().read_line(&mut token)?;
  let _r = auth.login_token(token.trim_end())
    .initial_device_display_name("mscript")
    .await?;

  let login_info = LoginInfo {
    homeserver: client.homeserver(),
    user_id: String::from(client.user_id().unwrap().as_str()),
    device_id: String::from(client.device_id().unwrap().as_str()),
    access_token: client.access_token().unwrap(),
  };
  let data = serde_json::to_string_pretty(&login_info)?;
  let mut f = std::fs::File::create(loginfile)?;
  f.write_all(data.as_bytes())?;
  f.write_all(b"\n")?;

  Ok(client)
}

pub async fn get_client<P: AsRef<Path>>(logininfo: P) -> Result<Client> {
  info!("Login...");
  let f = std::fs::File::open(logininfo)?;
  let f = std::io::BufReader::new(f);
  let info: LoginInfo = serde_json::from_reader(f)?;

  let client = build_client(None, Some(&info.homeserver)).await?;

  let session = MatrixSession {
    meta: SessionMeta {
      user_id: info.user_id.try_into()?,
      device_id: info.device_id.into(),
    },
    tokens: SessionTokens {
      access_token: info.access_token,
      refresh_token: None,
    },
  };
  client.restore_session(session).await?;

  Ok(client)
}

async fn build_client(
  server_name: Option<&ruma::ServerName>,
  homeserver: Option<&url::Url>,
) -> Result<Client> {
  use matrix_sdk::encryption::{EncryptionSettings, BackupDownloadStrategy};
  let mut builder = Client::builder()
    .sqlite_store("states", None)
    .with_encryption_settings(EncryptionSettings {
        auto_enable_cross_signing: true,
        auto_enable_backups: true,
        backup_download_strategy: BackupDownloadStrategy::AfterDecryptionFailure,
    });
  if let Some(homeserver) = homeserver {
    builder = builder.homeserver_url(homeserver);
  } else if let Some(server_name) = server_name {
    builder = builder.server_name(server_name);
  }
  let client = builder.build().await?;

  Ok(client)
}
