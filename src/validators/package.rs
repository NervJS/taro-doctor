use std::process::Command;

use jsonschema::ValidationError;
use regex::Regex;
use serde_json::{self, from_str, Value};

use crate::validators::common::get_package_info;

use super::{
  common::Validator,
  message::{Message, MessageKind},
};

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

pub struct PackageValidator<'a> {
  pub json: Value,
  pub node_modules_path: &'a str,
}

impl<'a> PackageValidator<'a> {
  pub fn build(
    package_str: &str,
    node_modules_path: &'a str,
  ) -> Result<Self, ValidationError<'static>> {
    let package_json = from_str(package_str)?;
    Ok(Self {
      json: package_json,
      node_modules_path,
    })
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
    messages.push(Message {
      kind: MessageKind::Manual,
      content: "本地安装的 Taro 相关依赖版本信息如下：".to_string(),
      solution: None,
    });
    let local_cli = get_package_info(self.node_modules_path, "@tarojs/cli");
    let mut _is_use_local = false;
    let cli_version = match local_cli {
      Ok(info) => {
        _is_use_local = true;
        info.version
      }
      Err(_) => {
        let output = Command::new("taro").arg("--version").output();
        let version = match output {
          Ok(output) => {
            let mut version = "".to_string();
            if output.status.success() {
              let output_str = String::from_utf8_lossy(&output.stdout);
              let parts: Vec<&str> = output_str.as_ref().split('\n').collect();
              let re = Regex::new(r"v(\d+\.\d+\.\d+)").unwrap();
              for p in parts.into_iter() {
                if let Some(captures) = re.captures(p) {
                  if let Some(v) = captures.get(1) {
                    version = v.as_str().to_string();
                    break;
                  }
                }
              }
              version
            } else {
              version
            }
          }
          Err(_) => "".to_string(),
        };
        _is_use_local = false;
        version
      }
    };
    if _is_use_local {
      messages.push(Message {
        kind: MessageKind::Warning,
        content: format!(
          "本地已经安装了 Taro CLI 版本为 {}，建议使用 npm script 来执行项目的预览和打包",
          cli_version
        ),
        solution: None,
      });
    }

    for p in taro_packages {
      let package_info = get_package_info(self.node_modules_path, p);
      match package_info {
        Ok(info) => {
          messages.push(Message {
            kind: MessageKind::Manual,
            content: format!("- {}: {}", info.name, info.version),
            solution: None,
          });
          if !cli_version.is_empty()
            && UPDATE_PACKAGE_LIST.contains(&p.as_str())
            && cli_version != info.version
          {
            messages.push(Message {
              kind: MessageKind::Error,
              content: format!(
                "依赖 {} ({}) 与当前使用的 Taro CLI ({}) 版本不一致, 请更新为统一的版本",
                p, info.version, cli_version
              ),
              solution: None,
            });
          }
        }
        Err(_) => {
          messages.push(Message {
            kind: MessageKind::Error,
            content: format!("请安装 Taro 依赖: {}", p),
            solution: None,
          });
        }
      }
    }
    messages.sort_by_key(|message| match message.kind {
      MessageKind::Info => 0,
      MessageKind::Warning => 1,
      MessageKind::Manual => 2,
      MessageKind::Success => 3,
      MessageKind::Error => 4,
    });
    messages
  }
}
