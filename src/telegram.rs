use matrix_sdk::ruma;

static TELEGRAM_BRIDGES: &[&str] = &[
  "nichi.co",
  "t2bot.io",
  "elv.sh",
  "moe.cat",
  "neo.angry.im",
];

static TELEGRAM_BRIDGES_2: &[&str] = &[
  "tether.kimiblock.top",
];

pub fn is_telegram(uid: &ruma::UserId) -> bool {
  (TELEGRAM_BRIDGES.contains(&uid.server_name().as_str())
   && uid.localpart().starts_with("telegram_"))
  || (TELEGRAM_BRIDGES_2.contains(&uid.server_name().as_str())
    && uid.localpart().starts_with("tg_"))
}

pub fn is_telegram_str(uid: &str) -> bool {
  if let Some((user, server)) = uid.rsplit_once(':') {
    (TELEGRAM_BRIDGES.contains(&server) && user.starts_with("@telegram_"))
      || (TELEGRAM_BRIDGES_2.contains(&server) && user.starts_with("@tg_"))
  } else {
    false
  }
}
