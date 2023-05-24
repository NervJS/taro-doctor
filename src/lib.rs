#![deny(clippy::all)]

mod validators;

use std::{ fs, error::Error, path::PathBuf, env };

use crate::validators::{ message::{ Message, self }, package::PackageValidator, config::ConfigValidator, common::Validator };

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn validate_config(config_str: String) {
  let result = validate_config_core(config_str);
  if let Err(e) = result {
    println!("{}", Message { kind: message::MessageKind::Error, content: e.to_string() });
  }
}

#[napi]
pub fn validate_package(app_path: String, node_modules_path: String) {
  let result = validate_package_core(app_path, node_modules_path);
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

fn validate_package_core(app_path: String, node_modules_path: String) -> Result<(), Box<dyn Error>> {
  let tip = Message {
    kind: validators::message::MessageKind::Info,
    content: String::from("开始进行项目依赖安装正确性验证！")
  };
  println!("{}", tip);
  let mut path = PathBuf::new();
  path.push(app_path);
  path.push("package.json");
  let package_str = fs::read_to_string(path.as_path())?;
  let package_validator_result = PackageValidator::build(&package_str, &node_modules_path).unwrap();
  package_validator_result.get_taro_packages();
  Ok(())
}
