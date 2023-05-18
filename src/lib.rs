#![deny(clippy::all)]

mod validators;

use std::{fs, error::Error, path::PathBuf, env};

use validators::{ config::ConfigValidator, common::Validator };

use crate::validators::message::{Message, self};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn validate_config(config_str: String) {
  let result = validate_config_core(config_str);
  if let Err(e) = result {
    println!("{}", Message { kind: message::MessageKind::Error, content: e.to_string() });
  }
}

fn validate_config_core(config_str: String) -> Result<(), Box<dyn Error>> {
  let tip = Message {
    kind: validators::message::MessageKind::Info,
    content: String::from("开始进行项目配置验证！")
  };
  println!("{}", tip);
  let current_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let mut path = PathBuf::new();
  path.push(current_dir);
  path.push("src");
  path.push("validators");
  path.push("config_schema.json");
  let schema_path = path.as_path();
  let schema_str = fs::read_to_string(schema_path)?;
  let config_validator_result = ConfigValidator::build(schema_str, config_str);
  let messages = match config_validator_result {
    Ok(config_validator) => config_validator.validate(),
    Err(e) => vec![
      Message {
        kind: validators::message::MessageKind::Error,
        content: e.to_string()
      }
    ]
  };
  if messages.len() > 0 {
    for message in messages {
      println!("{}", message);
    }
  } else {
    println!("{}", Message { kind: message::MessageKind::Success, content: "项目配置正确！".to_string() });
  }
  Ok(())
}
