#![deny(clippy::all)]

mod validators;

use std::{fs, error::Error, path::PathBuf, env};

use validators::{ config::ConfigValidator, common::Validator };

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn validate_config(config_str: String) -> String {
  let result = validate_config_core(config_str);
  if let Err(e) = result {
    println!("error, {}", e);
  }
  "Success".to_string()
}

fn validate_config_core(config_str: String) -> Result<(), Box<dyn Error>> {
  let current_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let mut path = PathBuf::new();
  path.push(current_dir);
  path.push("src");
  path.push("validators");
  path.push("config_schema.json");
  let schema_path = path.as_path();
  println!("{}", schema_path.to_str().unwrap());
  let schema_str = fs::read_to_string(schema_path)?;
  if let Ok(config_validator) = ConfigValidator::build(schema_str, config_str) {
    let result = config_validator.validate();
    for message in result {
      println!("{}", message);
    }
  }
  Ok(())
}