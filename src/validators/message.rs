use std::{fmt, error::Error};

#[derive(Debug)]
pub enum MessageKind {
  Info,
  Error,
  Success,
  Manual
}

#[derive(Debug)]
pub struct Message {
  pub kind: MessageKind,
  pub content: String,
}

impl fmt::Display for Message {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.kind {
      MessageKind::Error => {
        write!(f, "{} {}", emojis::get("❌").unwrap(), self.content)
      },
      MessageKind::Info => {
        write!(f, "{} {}", emojis::get("🎯").unwrap(), self.content)
      },
      MessageKind::Success => {
        write!(f, "{} {}", emojis::get("✅").unwrap(), self.content)
      },
      MessageKind::Manual => {
        write!(f, "{}", self.content)
      }
    }
  }
}

impl Error for Message {
  fn description(&self) -> &str {
    &self.content
  }
}
