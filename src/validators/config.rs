use serde_json::{ self, Value, from_str };
use jsonschema::{ JSONSchema, ValidationError, error::{ ValidationErrorKind, TypeKind }};

use super::{ message::{ Message, MessageKind }, common::Validator };

pub struct ConfigValidator {
  pub schema: Value,
  pub config: Value,
}

impl ConfigValidator {
  pub fn build(schema: String, config: String) -> Result<Self, ValidationError<'static>> {
    let schema_json = from_str(&schema)?;
    let config_json = from_str(&config)?;
    Ok(Self { schema: schema_json, config: config_json })
  }

  fn parse_error(&self, error: ValidationError) -> String {
    let instance_path = error.instance_path
      .clone()
      .into_vec()
      .iter()
      .map(|x| format!("{}", x))
      .collect::<Vec<String>>()
      .join(".");
    match error.kind {
      ValidationErrorKind::Schema => "JSON Schema 错误".to_string(),
      ValidationErrorKind::Format { format } => format!(r#"{} 的值 {} 不是 "{}" 格式"#, instance_path, error.instance, format),
      ValidationErrorKind::Not { schema } => format!("{} 的值 {} 不应符合 {}", instance_path, error.instance, schema),
      ValidationErrorKind::AdditionalItems { limit } => {
        let extras: Vec<&Value> = error
          .instance
          .as_array()
          .expect("Always valid")
          .iter()
          .skip(limit)
          .collect();
        format!(
          "{} 的取值超出数组长度限制 ({} 或许超出预期)",
          instance_path,
          extras
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
        )
      },
      ValidationErrorKind::AdditionalProperties { unexpected } => {
        format!(
          "{} 的取值为对象类型，不允许包含以下属性 {}",
          instance_path,
          unexpected
            .iter()
            .map(|x| format!("'{}'", x))
            .collect::<Vec<String>>()
            .join(", ")
        )
      },
      ValidationErrorKind::Resolver { url, error } => format!("解析错误 {}: {}", url, error),
      ValidationErrorKind::BacktrackLimitExceeded { error } => error.to_string(),
      ValidationErrorKind::JSONParse { error } => error.to_string(),
      ValidationErrorKind::InvalidURL { error } => error.to_string(),
      ValidationErrorKind::FileNotFound { error } => error.to_string(),
      ValidationErrorKind::FromUtf8 { error } => error.to_string(),
      ValidationErrorKind::Utf8 { error } => error.to_string(),
      ValidationErrorKind::UnknownReferenceScheme { scheme } => format!("未知的 schema {}", scheme),
      ValidationErrorKind::AnyOf | ValidationErrorKind::OneOfNotValid => format!("{} 的值 {} 不符合类型要求", instance_path, error.instance),
      ValidationErrorKind::Contains => format!("{} 中的每一项均不属于合法的配置项", error.instance),
      ValidationErrorKind::Constant { expected_value } => format!("{} 的值期望是 {}", instance_path, expected_value),
      ValidationErrorKind::ContentEncoding { content_encoding } => format!(r#"{} 的值 {} 不符合要求的内容编码格式 "{}""#, instance_path, error.instance, content_encoding),
      ValidationErrorKind::ContentMediaType { content_media_type } => format!(r#" {} 的值 {} 不符合要求的MIME 类型 "{}""#, instance_path, error.instance, content_media_type),
      ValidationErrorKind::Enum { options } => format!("{} 的值 {} 与任何指定选项 {} 都不匹配", instance_path, error.instance, options),
      ValidationErrorKind::ExclusiveMaximum { limit } => format!("{} 的值 {} 超出或等于最大值 {}", instance_path, error.instance, limit),
      ValidationErrorKind::ExclusiveMinimum { limit } => format!("{} 的值 {} 小于或等于最小值 {}", instance_path, error.instance, limit),
      ValidationErrorKind::FalseSchema => format!("False schema does not allow {}", error.instance),
      ValidationErrorKind::InvalidReference { reference } => format!("Invalid reference: {}", reference),
      ValidationErrorKind::Maximum { limit } => format!("{} 的值 {} 比最大值 {} 大", instance_path, error.instance, limit),
      ValidationErrorKind::Minimum { limit } => format!("{} 的值 {} 比最小值 {} 小", instance_path, error.instance, limit),
      ValidationErrorKind::MaxLength { limit } => format!(
        "{} 的值 {} 超出 {} 个字母",
        instance_path,
        error.instance,
        limit
      ),
      ValidationErrorKind::MinLength { limit } => format!(
        "{} 的值 {} 少于 {} 个字母",
        instance_path,
        error.instance,
        limit
      ),
      ValidationErrorKind::MaxItems { limit } => format!(
        "{} 的值 {} 超出数组最大长度 {}",
        instance_path,
        error.instance,
        limit
      ),
      ValidationErrorKind::MinItems { limit } => format!(
        "{} 的值 {} 小于数组要求长度 {}",
        instance_path,
        error.instance,
        limit
      ),
      ValidationErrorKind::MaxProperties { limit } => format!(
        "{} 的值 {} 超出对象最大属性个数 {}",
        instance_path,
        error.instance,
        limit
      ),
      ValidationErrorKind::MinProperties { limit } => format!(
        "{} 的值 {} 少于对象最小属性个数 {}",
        instance_path,
        error.instance,
        limit
      ),
      ValidationErrorKind::OneOfMultipleValid => format!(
        "{} 的值 {} 不满足要求的格式",
        instance_path,
        error.instance
      ),
      ValidationErrorKind::Pattern { pattern } => format!(r#"{} 的值 {} 与 "{}" 不匹配"#, instance_path, error.instance, pattern),
      ValidationErrorKind::PropertyNames { error } => error.to_string(),
      ValidationErrorKind::Required { property } => format!("缺少 {} 配置项", property),
      ValidationErrorKind::MultipleOf { multiple_of } => format!("{} 的值 {} 不是 {}的倍数", instance_path, error.instance, multiple_of),
      ValidationErrorKind::UnevaluatedProperties { unexpected } => {
        format!(
          "{} 的值中存在未被验证的属性 {}",
          instance_path,
          unexpected
            .iter()
            .map(|x| format!("'{}'", x))
            .collect::<Vec<String>>()
            .join(", ")
        )
      }
      ValidationErrorKind::UniqueItems => format!("{} 的值 {} 中存在重复的数组元素", instance_path, error.instance),
      ValidationErrorKind::Type {
        kind: TypeKind::Single(type_),
      } => format!(r#"{} 的值 {} 不是 "{}" 类型"#, instance_path, error.instance, type_),
      ValidationErrorKind::Type {
        kind: TypeKind::Multiple(types),
      } => format!(
        "{} 的值 {} 不属于类型 {} 中的任何一种",
        instance_path,
        error.instance,
        types
          .into_iter()
          .map(|t| format!(r#""{}""#, t))
          .collect::<Vec<String>>()
          .join(", ")
      )
    }
  }
}

impl Validator for ConfigValidator {
  fn validate(&self) -> Vec<Message> {
    let mut errors_result = Vec::new();

    match JSONSchema::compile(&self.schema) {
      Ok(compiled) => {
        if let Err(errors) = compiled.validate(&self.config) {
          for error in errors {
            // errors_result.push(ValidationError {
            //   instance_path: error.instance_path.clone(),
            //   instance: Cow::Owned(error.instance.into_owned()),
            //   kind: error.kind,
            //   schema_path: error.schema_path,
            // })
            errors_result.push(Message {
              kind: MessageKind::Error,
              content: self.parse_error(error),
              solution: None
            });
          }
        }
      }
      Err(error) => {
        errors_result.push(Message { kind: MessageKind::Error, content: self.parse_error(error), solution: None });
      },
    }

    errors_result
  }
}
