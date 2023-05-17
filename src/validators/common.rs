use super::message::Message;

pub trait Validator {
  fn validate(&self) -> Vec<Message>;
}
