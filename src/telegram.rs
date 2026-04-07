use std::sync::LazyLock;
use std::collections::HashMap;

use matrix_sdk::ruma;

static TELEGRAM_BRIDGES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
  let mut m = HashMap::new();
  m.insert("nichi.co", "telegram_");
  m.insert("t2bot.io", "telegram_");
  m.insert("elv.sh", "telegram_");
  m.insert("moe.cat", "telegram_");
  m.insert("kimiblock.top", "telegram_");
  m.insert("neo.angry.im", "perigram_");
  m.insert("tether.kimiblock.top", "tg_");
  m
});

pub fn is_telegram(uid: &ruma::UserId) -> bool {
  get_telegram_id(uid).is_some()
}

pub fn get_telegram_id(uid: &ruma::UserId) -> Option<&str> {
  let server = uid.server_name().as_str();
  TELEGRAM_BRIDGES.get(server).and_then(|prefix| uid.localpart().strip_prefix(prefix))
}

pub fn is_telegram_str(uid: &str) -> bool {
  ruma::UserId::parse(uid).ok().as_deref().and_then(get_telegram_id).is_some()
}
