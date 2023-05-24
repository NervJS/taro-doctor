use std::{ path::Path };

use jsonschema::ValidationError;
use serde_json::{ self, Value, from_str };

use super::{ message::{ Message, MessageKind }, common::Validator };

pub struct PackageValidator<'a>{
  pub json: Value,
  pub node_modules_path: &'a str
}

impl<'a> PackageValidator<'a> {
  pub fn build(package_str: &str, node_modules_path: &'a str) -> Result<Self, ValidationError<'static>> {
    let package_json = from_str(package_str)?;
    Ok(Self { json: package_json, node_modules_path })
  }

  pub fn get_taro_packages(&self) {
    let dependencies = self.json.get("dependencies").unwrap();
    let dev_dependencies = self.json.get("devDependencies").unwrap();
    let mut taro_packages = vec![];
    if let Value::Object(dependencies_map) = dependencies {
      for (key, _) in dependencies_map {
        if key.contains("@tarojs/") {
          taro_packages.push(key);
        }
      }
    }

    if let Value::Object(dev_dependencies_map) = dev_dependencies {
      for (key, _) in dev_dependencies_map {
        if key.contains("@tarojs/") {
          taro_packages.push(key);
        }
      }
    }
    
    println!("{:?}", taro_packages);
  }
}

// impl Validator for PackageValidator {
//   fn validate(&self) -> Vec<Message> {

//   }
// }
