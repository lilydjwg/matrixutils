use matrix_sdk::Room;

pub struct ShowRoom<'a>(pub &'a Room);

use std::fmt;

impl fmt::Display for ShowRoom<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    if let Some(alias) = self.0.canonical_alias() {
      write!(f, "{}", alias)
    } else {
      write!(f, "{}", self.0.room_id())
    }
  }
}
