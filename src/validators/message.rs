use std::{error::Error, fmt};

use console::style;
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};

#[derive(Debug, PartialEq)]
#[napi]
pub enum MessageKind {
  Info,
  Error,
  Success,
  Warning,
  Manual,
}

#[derive(Debug)]
#[napi(object)]
pub struct Message {
  pub kind: MessageKind,
  pub content: String,
  pub solution: Option<String>,
}

#[napi(object)]
pub struct ValidateResult {
  pub is_valid: bool,
  pub messages: Vec<Message>,
}

impl fmt::Display for Message {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.kind {
      MessageKind::Error => {
        if let Some(solution) = &self.solution {
          write!(
            f,
            "{} {}     {}",
            style("[✗] ").red(),
            style(&self.content).white(),
            style(solution).color256(246)
          )
        } else {
          write!(
            f,
            "{} {}",
            style("[✗] ").red(),
            style(&self.content).white()
          )
        }
      }
      MessageKind::Info => {
        write!(
          f,
          "{} {}",
          emojis::get("🎯").unwrap(),
          style(&self.content).color256(248).bold()
        )
      }
      MessageKind::Success => {
        write!(
          f,
          "{} {}",
          style("[✓] ").green(),
          style(&self.content).white()
        )
      }
      MessageKind::Warning => {
        if let Some(solution) = &self.solution {
          write!(
            f,
            "{} {}     {}",
            style("[!] ").yellow(),
            style(&self.content).white(),
            style(solution).color256(246)
          )
        } else {
          write!(
            f,
            "{} {}",
            style("[!] ").yellow(),
            style(&self.content).white()
          )
        }
      }
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
