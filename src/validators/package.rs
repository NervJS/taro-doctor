use std::{ cmp::Ordering };

use jsonschema::ValidationError;
use serde_json::{ self, Value, from_str };

use crate::validators::common::get_package_info;

use super::{ message::{ Message, MessageKind }, common::Validator };

const UPDATE_PACKAGE_LIST: [&str; 53] = [
  "babel-plugin-transform-react-jsx-to-rn-stylesheet",
  "taro-css-to-react-native",
  "stylelint-config-taro-rn",
  "stylelint-taro-rn",
  "babel-plugin-transform-taroapi",
  "babel-preset-taro",
  "eslint-config-taro",
  "postcss-html-transform",
  "postcss-plugin-constparse",
  "postcss-pxtransform",
  "@tarojs/shared",
  "@tarojs/taro",
  "@tarojs/cli",
  "@tarojs/api",
  "@tarojs/components",
  "@tarojs/components-react",
  "@tarojs/components-rn",
  "@tarojs/extend",
  "@tarojs/taro-h5",
  "@tarojs/taro-rn",
  "@tarojs/rn-runner",
  "@tarojs/rn-style-transformer",
  "@tarojs/rn-supporter",
  "@tarojs/rn-transformer",
  "@tarojs/helper",
  "@tarojs/taro-loader",
  "@tarojs/mini-runner",
  "@tarojs/react",
  "@tarojs/plugin-framework-react",
  "@tarojs/plugin-framework-vue2",
  "@tarojs/plugin-framework-vue3",
  "@tarojs/plugin-react-devtools",
  "@tarojs/plugin-vue-devtools",
  "@tarojs/router",
  "@tarojs/router-rn",
  "@tarojs/runner-utils",
  "@tarojs/runtime",
  "@tarojs/runtime-rn",
  "@tarojs/service",
  "@tarojs/webpack-runner",
  "@tarojs/with-weapp",
  "@tarojs/taroize",
  "@tarojs/plugin-platform-weapp",
  "@tarojs/plugin-platform-alipay",
  "@tarojs/plugin-platform-swan",
  "@tarojs/plugin-platform-tt",
  "@tarojs/plugin-platform-qq",
  "@tarojs/plugin-platform-jd",
  "@tarojs/plugin-platform-h5",
  "@tarojs/plugin-html",
  "@tarojs/plugin-mini-ci",
  "@tarojs/webpack5-runner",
  "@tarojs/webpack5-prebundle",
];

pub struct PackageValidator<'a>{
  pub json: Value,
  pub node_modules_path: &'a str,
  pub cli_version: &'a str
}

impl<'a> PackageValidator<'a> {
  pub fn build(package_str: &str, node_modules_path: &'a str, cli_version: &'a str) -> Result<Self, ValidationError<'static>> {
    let package_json = from_str(package_str)?;
    Ok(Self { json: package_json, node_modules_path, cli_version })
  }

  pub fn get_taro_packages(&self) -> Vec<&String> {
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
    
    taro_packages
  }
}

impl<'a> Validator for PackageValidator<'a> {
  fn validate(&self) -> Vec<Message> {
    let mut messages: Vec<Message> = vec![];
    let taro_packages = self.get_taro_packages();
    messages.push(Message { kind: MessageKind::Manual, content: "本地安装的 Taro 相关依赖版本信息如下：".to_string() });
    let cli_version = self.cli_version;
    for p in taro_packages {
      let package_info = get_package_info(self.node_modules_path, p);
      match package_info {
        Ok(info) => {
          messages.push(Message { kind: MessageKind::Manual, content: format!("- {}: {}", info.name, info.version) });
          if UPDATE_PACKAGE_LIST.contains(&p.as_str()) && cli_version != info.version {
            messages.push(Message { kind: MessageKind::Error, content: format!("依赖 {} ({}) 与当前使用的 Taro CLI ({}) 版本不一致, 请更新为统一的版本", p, info.version, cli_version) });
          }
        },
        Err(_) => {
          messages.push(Message { kind: MessageKind::Error, content: format!("请安装 Taro 依赖: {}", p) });
        }
      }
    }
    messages.sort_by(|a, b| {
      match (&a.kind, &b.kind) {
        (MessageKind::Manual, MessageKind::Manual) => Ordering::Equal,
        (MessageKind::Manual, _) => Ordering::Less,
        (_, MessageKind::Manual) => Ordering::Greater,
        (MessageKind::Error, MessageKind::Error) => Ordering::Equal,
        (MessageKind::Error, _) => Ordering::Greater,
        (_, MessageKind::Error) => Ordering::Less,
        _ => Ordering::Equal,
      }
    });
    messages
  }
}
