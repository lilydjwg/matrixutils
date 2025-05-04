use futures::Future;
use eyre::Result;
use tracing::info;

pub mod login;
pub mod verification;
pub mod telegram;
pub mod room;

pub async fn ctrlc_wrapper<F>(main_fu: F) -> Result<()>
where F: Future<Output=Result<()>>
{
  use futures::future::Either;

  futures::pin_mut!(main_fu);
  let ctrl_c = tokio::signal::ctrl_c();
  futures::pin_mut!(ctrl_c);

  match futures::future::select(main_fu, ctrl_c).await {
    Either::Left((a, _)) => a?,
    Either::Right((b, _)) => b?,
  }

  Ok(())
}

use std::time::Duration;

use matrix_sdk_base::store::{StateStoreDataKey, StateStoreDataValue};
use matrix_sdk::{Client, ruma, config::SyncSettings};

pub async fn sync_once(client: &Client) -> Result<SyncSettings> {
  let sync_token = client.state_store().get_kv_data(StateStoreDataKey::SyncToken).await?;
  let mut sync_settings = SyncSettings::new()
    .timeout(Duration::from_secs(600))
    .set_presence(ruma::presence::PresenceState::Unavailable);
  if let Some(StateStoreDataValue::SyncToken(token)) = sync_token {
    sync_settings = sync_settings.token(token);
  }
  info!("Syncing once...");
  client.sync_once(sync_settings.clone()).await?;
  info!("Synced.");

  Ok(sync_settings)
}
