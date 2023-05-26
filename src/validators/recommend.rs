use std::{error::Error, collections::HashSet};

use serde_json::{ self, Value };

use super::{ common::{Validator, compare_versions, get_package_info}, message::{ Message, MessageKind } };

const TEST_FRAMEWORKS: [&str; 6] = ["jest", "mocha", "ava", "tape", "jesmine", "karma"];
const LINTERS: [&str; 4] = ["eslint", "jslint", "tslint", "jshint"];
const README: [&str; 3] = ["readme", "readme.md", "readme.markdown"];
const GITIGNORE: [&str; 1] = [".gitignore"];
const EDITORCONFIG: [&str; 1] = [".editorconfig"];

pub struct RecommendValidator<'a> {
  pub app_path: &'a str,
  pub json: Value,
}

impl<'a> RecommendValidator<'a> {
  pub fn build(app_path: &'a str, node_modules_path: &'a str) -> Result<Self, Box<dyn Error>> {
    let package_info = get_package_info(node_modules_path, "")?;
    Ok(
      Self {
        app_path,
        json: package_info.json
      }
    )
  }
}

impl<'a> Validator for RecommendValidator<'a> {
  fn validate(&self) -> Vec<Message> {
    let mut messages = vec![];
    // 读取 package.json
    let dev_dependencies = self.json.get("devDependencies").unwrap();
    let hasTestFramework = if let Value::Object(dev_dependencies_map) = dev_dependencies {
      let dev_dependencies: HashSet<_> = dev_dependencies_map.keys().map(|key| key.to_lowercase()).into_iter().collect();
      let test_frameworks: HashSet<_> = TEST_FRAMEWORKS.map(|key| key.to_lowercase()).into_iter().collect();
      let has_intersection = !test_frameworks.is_disjoint(&dev_dependencies);
    };
    messages
  }
}
