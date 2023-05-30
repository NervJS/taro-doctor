#![deny(clippy::all)]

mod validators;

use std::{error::Error, fs, path::PathBuf};

use validators::{env::EnvValidator, message::ValidateResult, recommend::RecommendValidator};

use crate::validators::{
  common::Validator,
  config::ConfigValidator,
  message::{Message, MessageKind},
  package::PackageValidator,
};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn validate_config(config_str: String) -> ValidateResult {
  let result = validate_config_core(config_str);
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages.iter() {
    if message.kind == MessageKind::Error {
      is_valid = false;
      break;
    }
  }
  ValidateResult { is_valid, messages }
}

#[napi]
pub fn validate_config_print(config_str: String) -> bool {
  let result = validate_config_core(config_str);
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages.iter() {
    println!("{}", message);
    if message.kind == MessageKind::Error {
      is_valid = false;
    }
  }
  println!("");
  is_valid
}

#[napi]
pub fn validate_package(app_path: String, node_modules_path: String) -> ValidateResult {
  let result = validate_package_core(app_path, node_modules_path);
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages.iter() {
    if message.kind == MessageKind::Error {
      is_valid = false;
      break;
    }
  }
  ValidateResult { is_valid, messages }
}

#[napi]
pub fn validate_package_print(app_path: String, node_modules_path: String) -> bool {
  let result = validate_package_core(app_path, node_modules_path);
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages {
    println!("{}", message);
    if message.kind == MessageKind::Error {
      is_valid = false;
    }
  }
  println!("");
  is_valid
}

#[napi]
pub fn validate_env() -> ValidateResult {
  let result = validate_env_core();
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages.iter() {
    if message.kind == MessageKind::Error {
      is_valid = false;
      break;
    }
  }
  ValidateResult { is_valid, messages }
}

#[napi]
pub fn validate_env_print() -> bool {
  let result = validate_env_core();
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages {
    println!("{}", message);
    if message.kind == MessageKind::Error {
      is_valid = false;
    }
  }
  println!("");
  is_valid
}

#[napi]
pub fn validate_recommend(app_path: String) -> ValidateResult {
  let result = validate_recommend_core(app_path);
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages.iter() {
    if message.kind == MessageKind::Error {
      is_valid = false;
      break;
    }
  }
  ValidateResult { is_valid, messages }
}

#[napi]
pub fn validate_recommend_print(app_path: String) -> bool {
  let result = validate_recommend_core(app_path);
  let messages = match result {
    Ok(messages) => messages,
    Err(e) => {
      let mut messages = vec![];
      messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      });
      messages
    }
  };
  let mut is_valid = true;
  for message in messages {
    println!("{}", message);
    if message.kind == MessageKind::Error {
      is_valid = false;
    }
  }
  println!("");
  is_valid
}

fn validate_config_core(config_str: String) -> Result<Vec<Message>, Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证项目配置 (/config/index.js) ！"),
    solution: None,
  };
  let schema_str = include_str!("../assets/config_schema.json");
  let config_validator_result = ConfigValidator::build(String::from(schema_str), config_str);
  let mut messages = match config_validator_result {
    Ok(config_validator) => config_validator.validate(),
    Err(e) => vec![Message {
      kind: MessageKind::Error,
      content: e.to_string(),
      solution: None,
    }],
  };
  if messages.len() == 0 {
    messages.push(Message {
      kind: MessageKind::Success,
      content: "项目配置正确！".to_string(),
      solution: None,
    })
  }
  messages.insert(0, tip);
  Ok(messages)
}

fn validate_package_core(
  app_path: String,
  node_modules_path: String,
) -> Result<Vec<Message>, Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证项目依赖安装正确性！"),
    solution: None,
  };
  let mut path = PathBuf::new();
  path.push(app_path);
  path.push("package.json");
  let package_str = fs::read_to_string(path.as_path())?;
  let package_validator_result = PackageValidator::build(&package_str, &node_modules_path);
  let mut messages = match package_validator_result {
    Ok(package_validator) => package_validator.validate(),
    Err(e) => vec![Message {
      kind: MessageKind::Error,
      content: e.to_string(),
      solution: None,
    }],
  };
  messages.insert(0, tip);
  Ok(messages)
}

fn validate_env_core() -> Result<Vec<Message>, Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证环境信息！"),
    solution: None,
  };
  let env_validator = EnvValidator::build();
  let mut messages = env_validator.validate();
  messages.insert(0, tip);
  Ok(messages)
}

fn validate_recommend_core(app_path: String) -> Result<Vec<Message>, Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证最佳实践！"),
    solution: None,
  };
  let recommend_validator_result = RecommendValidator::build(&app_path);
  let mut messages = match recommend_validator_result {
    Ok(recommend_validator) => recommend_validator.validate(),
    Err(e) => vec![Message {
      kind: MessageKind::Error,
      content: e.to_string(),
      solution: None,
    }],
  };
  if messages.len() == 0 {
    messages.push(Message {
      kind: MessageKind::Success,
      content: "项目符合最佳实践要求！".to_string(),
      solution: None,
    })
  }
  messages.insert(0, tip);
  Ok(messages)
}
