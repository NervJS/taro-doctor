use std::{cmp::Ordering, process::Command};

use super::{
  common::{compare_versions, Validator},
  message::{Message, MessageKind},
};

pub struct EnvValidator {}

impl EnvValidator {
  pub fn build() -> Self {
    Self {}
  }
}

impl Validator for EnvValidator {
  fn validate(&self) -> Vec<Message> {
    let mut messgaes = vec![];
    // 获取当前 node 版本
    let output = Command::new("node").arg("--version").output();
    let message = match output {
      Ok(output) => {
        if output.status.success() {
          let version = String::from_utf8_lossy(&output.stdout);
          if let Some(ordering) = compare_versions(
            version.as_ref().replace("v", "").replace("\n", "").as_str(),
            "14.0.0",
          ) {
            if ordering == Ordering::Greater || ordering == Ordering::Equal {
              Message {
                kind: MessageKind::Success,
                content: format!("安装的 Node 版本为 {}", version.replace("\n", "")),
                solution: None,
              }
            } else {
              Message {
                kind: MessageKind::Error,
                content: format!("安装的 Node 版本为 {}，小于最低要求 Node 版本 14.0.0", version),
                solution: Some("请安装正确的 Node 版本，推荐使用 nvm(https://github.com/nvm-sh/nvm) 来管理 Node 版本".to_string())
              }
            }
          } else {
            Message {
              kind: MessageKind::Success,
              content: format!("安装的 Node 版本为 {}", version),
              solution: None,
            }
          }
        } else {
          Message {
            kind: MessageKind::Error,
            content: format!("获取 Node 版本失败，请查看是否正确安装 Node"),
            solution: Some(
              "推荐使用 nvm(https://github.com/nvm-sh/nvm) 来管理 Node 版本".to_string(),
            ),
          }
        }
      }
      Err(_) => Message {
        kind: MessageKind::Error,
        content: format!("获取 Node 版本失败，请查看是否正确安装 Node"),
        solution: Some("推荐使用 nvm(https://github.com/nvm-sh/nvm) 来管理 Node 版本".to_string()),
      },
    };

    messgaes.push(message);

    messgaes
  }
}
