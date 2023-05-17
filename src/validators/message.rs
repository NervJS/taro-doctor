use std::fmt;

pub enum MessageKind {
  Info,
  Error,
  Success,
}

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
    }
  }
}
