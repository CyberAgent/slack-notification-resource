use error::Result as LibResult;
use chrono::prelude::*;
use std::fmt::Debug;
use serde_json::{self, Value};
use std::convert::From;

pub type UnixTimestamp = DateTime<Utc>;

pub trait TryFromIterator<A>: Sized {
    type Error;

    fn try_from_iter<I>(iter: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = A> + Debug;
}

// @see
// https://github.com/rust-lang/rust/issues/33417
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait Validatable {
    fn validate(&self) -> LibResult<()>;
}

pub trait Validator {
    fn validate(&self) -> LibResult<()>;
}

// @todo need wrap?
pub struct WrapValue(pub Value);

impl From<WrapValue> for Vec<String> {
    fn from(wrap_value: WrapValue) -> Self {
        fn gen(value: &Value, option: String) -> Vec<String> {
            match *value {
                Value::Bool(_) => vec![option],
                Value::String(ref s) => vec![option, s.to_string()],
                _ => vec![option, value.to_string()],
            }
        }

        fn go(prefix: Option<&str>, value: &Value) -> Vec<String> {
            let empty = serde_json::Map::new();
            let obj = value.as_object().unwrap_or(&empty);

            obj.iter()
                .flat_map(|(k, v)| {
                    let arg_name = prefix
                        .map(|p| format!("{}.{}", p, k))
                        .unwrap_or_else(|| k.to_string());

                    match v.as_object() {
                        None => gen(v, format!("--{}", &arg_name)),
                        Some(_) => go(Some(&arg_name), v),
                    }
                })
                .collect::<Vec<String>>()
        }
        go(None, &wrap_value.0)
    }
}

#[cfg(test)]
mod tests {

    use core::*;
    use serde_json::{self, Value};

    #[test]
    fn from_value_ok() {
        let json = r#"
            {
              "params": {
                "channel": "test-channel",
                "attachments": "test-attachments"
              },
              "source": {
                "token": "test-token"
              }
            }
            "#;
        let mut expect = vec![
            "--source.token",
            "test-token",
            "--params.attachments",
            "test-attachments",
            "--params.channel",
            "test-channel",
        ];
        let value = serde_json::from_str::<Value>(json).unwrap();
        let mut actual: Vec<String> = WrapValue(value).into();
        assert_eq!(actual.sort(), expect.sort());
    }

    #[test]
    fn from_value_multi_stage_ok() {
        let value = serde_json::from_str::<Value>(
            r#"
            {
              "a": {
                "b": {
                  "c": "value"
                }
              }
            }
            "#,
        ).unwrap();
        let expect = vec!["--a.b.c", "value"];
        let actual: Vec<String> = WrapValue(value).into();
        assert_eq!(actual, expect);
    }

    #[test]
    fn from_value_empty_ok() {
        let value = serde_json::from_str::<Value>("{}").unwrap();
        let expect: Vec<String> = Vec::new();
        let actual: Vec<String> = WrapValue(value).into();
        assert_eq!(actual, expect);
    }

}
