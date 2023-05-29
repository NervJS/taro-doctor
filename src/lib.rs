#![deny(clippy::all)]

mod validators;

use std::{ fs, error::Error, path::PathBuf, env, process::Command };

use validators::{env::EnvValidator, recommend::RecommendValidator};

use crate::validators::{ message::{ Message, MessageKind }, package::{ PackageValidator }, config::ConfigValidator, common::{ Validator } };

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn validate_config(config_str: String) {
  let result = validate_config_core(config_str);
  if let Err(e) = result {
    println!("{}", Message { kind: MessageKind::Error, content: e.to_string(), solution: None });
  }
}

#[napi]
pub fn validate_package(app_path: String, node_modules_path: String, cli_version: String) {
  let result = validate_package_core(app_path, node_modules_path, cli_version);
  if let Err(e) = result {
    println!("{}", Message { kind: MessageKind::Error, content: e.to_string(), solution: None });
  }
}

#[napi]
pub fn validate_env() {
  let result = validate_env_core();
  if let Err(e) = result {
    println!("{}", Message { kind: MessageKind::Error, content: e.to_string(), solution: None });
  }
}

#[napi]
pub fn validate_recommend(app_path: String) {
  let result = validate_recommend_core(app_path);
  if let Err(e) = result {
    println!("{}", Message { kind: MessageKind::Error, content: e.to_string(), solution: None });
  }
}

#[napi]
pub fn validate_eslint() {
  let mut command = Command::new("npm");
  command.arg("run");
  command.arg("eslint");

  let output = command.output().expect("failed to execute eslint");

  if output.status.success() {
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Command output: {}", stdout);
  } else {
    let stderr = String::from_utf8_lossy(&output.stdout);
    eprintln!("Command failed: {}", stderr);
  }
}

fn validate_config_core(config_str: String) -> Result<(), Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证项目配置配置！"),
    solution: None
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
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None
      }
    ]
  };
  if messages.len() > 0 {
    for message in messages {
      println!("{}", message);
    }
  } else {
    println!("{}", Message { kind: MessageKind::Success, content: "项目配置正确！".to_string(), solution: None });
  }
  Ok(())
}

fn validate_package_core(app_path: String, node_modules_path: String, cli_version: String) -> Result<(), Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证项目依赖安装正确性！"),
    solution: None
  };
  println!("{}", tip);
  let mut path = PathBuf::new();
  path.push(app_path);
  path.push("package.json");
  let package_str = fs::read_to_string(path.as_path())?;
  let package_validator_result = PackageValidator::build(&package_str, &node_modules_path, &cli_version);
  let messages = match package_validator_result {
    Ok(package_validator) => package_validator.validate(),
    Err(e) => vec![
      Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None
      }
    ]
  };
  for message in messages {
    println!("{}", message);
  }
  Ok(())
}

fn validate_env_core() -> Result<(), Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证环境信息！"),
    solution: None
  };
  println!("{}", tip);
  let env_validator = EnvValidator::build();
  let messages = env_validator.validate();
  for message in messages {
    println!("{}", message);
  }
  Ok(())
}

fn validate_recommend_core(app_path: String) -> Result<(), Box<dyn Error>> {
  let tip = Message {
    kind: MessageKind::Info,
    content: String::from("验证最佳实践！"),
    solution: None
  };
  println!("{}", tip);
  let recommend_validator_result = RecommendValidator::build(&app_path);
  let messages = match recommend_validator_result {
    Ok(recommend_validator) => recommend_validator.validate(),
    Err(e) => vec![
      Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None
      }
    ]
  };

  if messages.len() > 0 {
    for message in messages {
      println!("{}", message);
    }
  } else {
    println!("{}", Message { kind: MessageKind::Success, content: "项目符合最佳实践要求！".to_string(), solution: None });
  }
  Ok(())
}
