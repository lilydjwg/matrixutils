use std::sync::LazyLock;

use matrix_sdk::ruma;

static TELEGRAM_BRIDGES: LazyLock<Vec<&'static ruma::ServerName>> = LazyLock::new(|| vec![
  ruma::server_name!("nichi.co"),
  ruma::server_name!("t2bot.io"),
  ruma::server_name!("elv.sh"),
  ruma::server_name!("moe.cat"),
  ruma::server_name!("neo.angry.im"),
]);

pub fn is_telegram(uid: &ruma::UserId) -> bool {
  TELEGRAM_BRIDGES.contains(&uid.server_name())
    && uid.localpart().starts_with("telegram_")
}
