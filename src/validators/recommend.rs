use std::{
  collections::HashSet,
  error::Error,
  fs,
  path::{PathBuf, MAIN_SEPARATOR},
};

use serde_json::{self, Value};

use super::{
  common::{get_package_info, Validator},
  message::{Message, MessageKind},
};

const TEST_FRAMEWORKS: [&str; 6] = ["jest", "mocha", "ava", "tape", "jesmine", "karma"];
const LINTERS: [&str; 4] = ["eslint", "jslint", "tslint", "jshint"];
const README: [&str; 3] = ["readme", "readme.md", "readme.markdown"];
const GITIGNORE: [&str; 1] = [".gitignore"];
const EDITORCONFIG: [&str; 7] = [
  ".editorconfig",
  "editorconfig",
  ".prettierrc",
  ".prettierrc.json",
  ".prettierrc.yml",
  ".prettierrc.js",
  "prettier.config.js",
];

pub struct RecommendValidator<'a> {
  pub app_path: &'a str,
  pub json: Value,
}

impl<'a> RecommendValidator<'a> {
  pub fn build(app_path: &'a str) -> Result<Self, Box<dyn Error>> {
    let package_info = get_package_info(app_path, "")?;
    Ok(Self {
      app_path,
      json: package_info.json,
    })
  }
}

impl<'a> Validator for RecommendValidator<'a> {
  fn validate(&self) -> Vec<Message> {
    let mut messages = vec![];
    let is_intersecting = |a: &HashSet<_>, b: &HashSet<_>| !(*a).is_disjoint(b);
    // 读取 package.json
    let dev_dependencies = self.json.get("devDependencies").unwrap();
    let mut has_test_framework = false;
    let mut has_linter = false;
    if let Value::Object(dev_dependencies_map) = dev_dependencies {
      let dev_dependencies: HashSet<String> = dev_dependencies_map
        .keys()
        .map(|key| key.to_lowercase())
        .into_iter()
        .collect();
      let test_frameworks: HashSet<String> = TEST_FRAMEWORKS
        .map(|key| key.to_lowercase())
        .into_iter()
        .collect();
      let linters: HashSet<String> = LINTERS.map(|key| key.to_lowercase()).into_iter().collect();
      if is_intersecting(&dev_dependencies, &test_frameworks) {
        has_test_framework = true;
      }
      if is_intersecting(&dev_dependencies, &linters) {
        has_linter = true;
      }
    }

    if !has_test_framework {
      messages.push(
        Message {
          kind: MessageKind::Warning,
          content: String::from("没有检查到常见的测试依赖(jest/mocha/ava/tape/jesmine/karma), 配置测试可以帮助提升项目质量\n"),
          solution: Some(String::from("可以参考 https://github.com/NervJS/taro-ui-sample 项目, 其中已经包含了完整的测试配置与范例"))
        }
      )
    }

    if !has_linter {
      messages.push(
        Message {
          kind: MessageKind::Warning,
          content: String::from("没有检查到常见的 linter (eslint/jslint/jshint/tslint), 配置 linter 可以帮助提升项目质量\n"),
          solution: Some(String::from("Taro 还提供了定制的 ESLint 规则, 可以帮助开发者避免一些常见的问题. 使用 taro cli 创建新项目即可体验"))
        }
      )
    }

    let app_dir_read = fs::read_dir(self.app_path);
    match app_dir_read {
      Ok(app_dir) => {
        let mut file_list: Vec<PathBuf> = Vec::new();
        for entry in app_dir {
          match entry {
            Ok(entry) => {
              let path = entry.path();
              if path.is_file() {
                file_list.push(path);
              }
            }
            Err(e) => messages.push(Message {
              kind: MessageKind::Error,
              content: e.to_string(),
              solution: None,
            }),
          }
        }
        let mut has_readme = false;
        let mut has_gitignore = false;
        let mut has_editorconfig = false;
        let mut app_path = self.app_path.to_string();
        app_path.push(MAIN_SEPARATOR);
        let file_list: HashSet<String> = file_list
          .into_iter()
          .map(|key: PathBuf| {
            key
              .into_os_string()
              .into_string()
              .unwrap()
              .replace(&app_path, "")
              .to_lowercase()
          })
          .into_iter()
          .collect();
        let readme_list: HashSet<String> =
          README.map(|key| key.to_lowercase()).into_iter().collect();
        let gitignore_list: HashSet<String> = GITIGNORE
          .map(|key| key.to_lowercase())
          .into_iter()
          .collect();
        let editorconfig_list: HashSet<String> = EDITORCONFIG
          .map(|key| key.to_lowercase())
          .into_iter()
          .collect();

        if is_intersecting(&file_list, &readme_list) {
          has_readme = true;
        }
        if is_intersecting(&file_list, &gitignore_list) {
          has_gitignore = true;
        }
        if is_intersecting(&file_list, &editorconfig_list) {
          has_editorconfig = true;
        }

        if !has_readme {
          messages.push(
            Message {
              kind: MessageKind::Warning,
              content: String::from("没有检查到 Readme (readme/readme.md/readme.markdown), 编写 Readme 可以方便其他人了解项目"),
              solution: None
            }
          )
        }

        if !has_gitignore {
          messages.push(
            Message {
              kind: MessageKind::Warning,
              content: String::from("没有检查到 .gitignore 配置, 配置 .gitignore 以避免将敏感信息或不必要的内容提交到代码仓库"),
              solution: None
            }
          )
        }

        if !has_editorconfig {
          messages.push(
            Message {
              kind: MessageKind::Warning,
              content: String::from("没有检查到代码格式化配置 (editconfig/prettier), 可进行相关配置以统一项目成员编辑器的代码风格"),
              solution: None
            }
          )
        }
      }
      Err(e) => messages.push(Message {
        kind: MessageKind::Error,
        content: e.to_string(),
        solution: None,
      }),
    }

    messages
  }
}
