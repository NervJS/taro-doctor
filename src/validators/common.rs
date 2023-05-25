use std::{error::Error, path::PathBuf, fs, cmp::Ordering};

use serde_json::{from_str, Value};

use super::message::{Message, MessageKind};

pub trait Validator {
  fn validate(&self) -> Vec<Message>;
}

pub struct PackageInfo {
  pub name: String,
  pub version: String,
}

pub fn get_package_info(node_modules_path: &str, name: &str) -> Result<PackageInfo, Box<dyn Error>> {
  let mut package_path = PathBuf::new();
  let mut name_arr = vec![];
  if name.contains('/') {
    name_arr = name.split('/').collect();
  } else {
    name_arr.push(name);
  }
  package_path.push(node_modules_path);
  for item in name_arr {
    package_path.push(item);
  }
  package_path.push("package.json");
  let package_str = fs::read_to_string(package_path.as_path())?;
  let package_json_re = from_str::<Value>(&package_str);
  match package_json_re {
    Ok(package_json) => {
      Ok(PackageInfo { name: name.to_string(), version: package_json.get("version").unwrap().as_str().unwrap().to_string() })
    },
    Err(e) => {
      Err(
        Box::new(
          Message {
            kind: MessageKind::Error,
            content: e.to_string()
          }
        )
      )
    }
  }
}

pub fn compare_versions(a: &str, b: &str) -> Option<Ordering> {
  let parts1: Vec<u64> = a.split('.').filter_map(|s| s.parse().ok()).collect();
  let parts2: Vec<u64> = b.split('.').filter_map(|s| s.parse().ok()).collect();
  for (p1, p2) in parts1.iter().zip(parts2.iter()) {
    match p1.cmp(p2) {
      Ordering::Less => return Some(Ordering::Less),
      Ordering::Greater => return Some(Ordering::Greater),
      Ordering::Equal => continue,
    }
  }

  match parts1.len().cmp(&parts2.len()) {
    Ordering::Less => Some(Ordering::Less),
    Ordering::Greater => Some(Ordering::Greater),
    Ordering::Equal => Some(Ordering::Equal),
  }
}
