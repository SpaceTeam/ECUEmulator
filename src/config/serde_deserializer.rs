use crate::config::config_representation::{Parameter, TelemetryValue};
use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

pub mod max_bytes {
    use super::*;
    use serde::de;

    pub fn deserialize<'de, const MAX: usize, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() > MAX {
            Err(de::Error::custom(format!(
                "string length {} exceeds maximum of {MAX} bytes",
                s.len()
            )))
        } else {
            Ok(s)
        }
    }
}

pub fn deserialize_value_or_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ValueOrBool {
        U32Value(#[serde(deserialize_with = "deserialize_prefixed_u32")] u32),
        BoolValue(bool),
    }

    let value = ValueOrBool::deserialize(deserializer)?;
    match value {
        ValueOrBool::U32Value(v) => Ok(v),
        ValueOrBool::BoolValue(b) => Ok(if b { 1 } else { 0 }),
    }
}

pub fn deserialize_telemetry<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<TelemetryValue>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: Option<HashMap<String, TelemetryValue>> = Option::deserialize(deserializer)?;
    Ok(map.map(|m| {
        m.into_iter()
            .map(|(name, mut var)| {
                var.name = name;
                var
            })
            .collect()
    }))
}

pub fn deserialize_parameters<'de, D>(deserializer: D) -> Result<Option<Vec<Parameter>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: Option<HashMap<String, Parameter>> = Option::deserialize(deserializer)?;
    Ok(map.map(|m| {
        m.into_iter()
            .map(|(name, mut param)| {
                param.name = name;
                param
            })
            .collect()
    }))
}

fn parse_prefixed_biguint(s: &str) -> Result<BigUint, String> {
    let s = s.replace('_', "");
    if s.starts_with("-") {
        return Err(format!("negative values are not supported. Value: {}", s));
    }
    if s.starts_with("0x") || s.starts_with("0X") {
        BigUint::parse_bytes(&s[2..].as_bytes(), 16).ok_or_else(|| {
            format!(
                "detected prefix '0x' but failed to parse value. Value: {}",
                s
            )
        })
    } else if s.starts_with("0b") || s.starts_with("0B") {
        BigUint::parse_bytes(&s[2..].as_bytes(), 2).ok_or_else(|| {
            format!(
                "detected prefix '0b' but failed to parse value. Value: {}",
                s
            )
        })
    } else {
        BigUint::parse_bytes(s.as_bytes(), 10).ok_or_else(|| {
            format!(
                "expecting decimal string but failed to parse value. Value: {}",
                s
            )
        })
    }
}
pub fn deserialize_prefixed_biguint<'de, D>(deserializer: D) -> Result<BigUint, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt::Formatter;
    struct PrefixedBigUintVisitor;
    impl<'de> serde::de::Visitor<'de> for PrefixedBigUintVisitor {
        type Value = BigUint;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            write!(formatter, "an unsigned big integer or a string starting with '0x', '0X', '0b','0B', or a decimal")
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v < 0 {
                return Err(E::custom("negative values are not supported for BigUint"));
            }
            BigUint::from_u64(v as u64).ok_or_else(|| E::custom("failed to convert i64 to BigUint"))
        }
        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            BigUint::from_u8(v).ok_or_else(|| E::custom("failed to convert u8 to BigUint"))
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: Error,
        {
            BigUint::from_u16(v).ok_or_else(|| E::custom("failed to convert u16 to BigUint"))
        }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            BigUint::from_u32(v).ok_or_else(|| E::custom("failed to convert u32 to BigUint"))
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            BigUint::from_u64(v).ok_or_else(|| E::custom("failed to convert u64 to BigUint"))
        }
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Err(E::custom(
                "floating point values are not supported for BigUint",
            ))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            parse_prefixed_biguint(v)
                .map_err(|e| E::custom(format!("failed to parse string to BigUint: {}", e)))
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
    }

    deserializer.deserialize_any(PrefixedBigUintVisitor)
}
pub fn deserialize_prefixed_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_prefix_generic_u32(deserializer, "u32")
}

fn deserialize_prefix_generic_u32<'de, D>(deserializer: D, type_str: &str) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::fmt::Formatter;
    struct PrefixedU32Visitor {
        type_str: String,
    }

    impl<'de> serde::de::Visitor<'de> for PrefixedU32Visitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            write!(formatter, "an unsigned {} integer or a string starting with '0x', '0X', '0b','0B', or a decimal", self.type_str)
        }
        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v < 0 {
                return Err(E::custom(format!(
                    "negative values are not supported for {}",
                    self.type_str
                )));
            }
            Ok(v as u32)
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v < 0 || v > u32::MAX as i64 {
                return Err(E::custom(format!(
                    "value is out of range for {}",
                    self.type_str
                )));
            }
            Ok(v as u32)
        }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            if v > u32::MAX as u64 {
                return Err(E::custom(format!(
                    "value exceeds {} maximum",
                    self.type_str
                )));
            }
            Ok(v as u32)
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let big_uint = parse_prefixed_biguint(v)
                .map_err(|e| E::custom(format!("failed to parse string to u32: {}", e)))?;
            big_uint
                .to_u32()
                .ok_or_else(|| E::custom(format!("parsed value does not fit in {}", self.type_str)))
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
    }

    deserializer.deserialize_any(PrefixedU32Visitor {
        type_str: type_str.to_string(),
    })
}

pub fn deserialize_prefixed_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserialize_prefix_generic_u32(deserializer, "u8")
        .map(|v: u32| {
            if v > u8::MAX as u32 {
                Err(Error::custom("value exceeds u8 maximum"))
            } else {
                Ok(v as u8)
            }
        })
        .and_then(|res| res)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_toml_biguint() {
        #[derive(serde::Deserialize, Debug)]
        struct TestConfig {
            #[serde(deserialize_with = "deserialize_prefixed_biguint")]
            value: BigUint,
        }

        let toml_data = r#"
            value = "0x1A2B3C"
        "#;

        let config: TestConfig = toml::from_str(toml_data).expect("Failed to deserialize TOML");
        assert_eq!(config.value, BigUint::from(0x1A2B3C_u64));

        let toml_data_bin = r#"
            value = "0b1101"
        "#;

        let config_bin: TestConfig =
            toml::from_str(toml_data_bin).expect("Failed to deserialize TOML");
        assert_eq!(config_bin.value, BigUint::from(13_u64));

        let toml_data_dec = r#"
            value = "123456789"
        "#;

        let config_dec: TestConfig =
            toml::from_str(toml_data_dec).expect("Failed to deserialize TOML");
        assert_eq!(config_dec.value, BigUint::from(123456789_u64));

        let toml_data_num = r#"
            value = 987654321
        "#;
        let config_num: TestConfig =
            toml::from_str(toml_data_num).expect("Failed to deserialize TOML");
        assert_eq!(config_num.value, BigUint::from(987654321_u64));
    }

    #[test]
    fn test_deserialize_toml_non_decimal() {
        #[derive(serde::Deserialize, Debug)]
        struct TestConfig {
            #[serde(deserialize_with = "deserialize_prefixed_biguint")]
            value: BigUint,
        }

        let toml_data_neg = r#"
            value = "-12345"
        "#;

        let result: Result<TestConfig, _> = toml::from_str(toml_data_neg);
        assert!(result.is_err(), "Expected error for negative value");

        let toml_data_float = r#"
            value = 12.34
        "#;

        let result_float: Result<TestConfig, _> = toml::from_str(toml_data_float);
        assert!(
            result_float.is_err(),
            "Expected error for floating point value"
        );
    }

    #[test]
    fn test_deserialize_toml_u32() {
        #[derive(serde::Deserialize, Debug)]
        struct TestConfig {
            #[serde(deserialize_with = "deserialize_prefixed_u32")]
            value: u32,
        }
        let toml_data = r#"
            value = "0x1A2B3C"
        "#;
        let config: TestConfig = toml::from_str(toml_data).expect("Failed to deserialize TOML");
        assert_eq!(config.value, 0x1A2B3C_u32);

        let toml_data_bin = r#"
            value = "0b1101"
        "#;
        let config_bin: TestConfig =
            toml::from_str(toml_data_bin).expect("Failed to deserialize TOML");
        assert_eq!(config_bin.value, 13_u32);

        let toml_data_dec = r#"
            value = "123456789"
        "#;
        let config_dec: TestConfig =
            toml::from_str(toml_data_dec).expect("Failed to deserialize TOML");
        assert_eq!(config_dec.value, 123456789_u32);

        let toml_data_num = r#"
            value = 987654321
        "#;
        let config_num: TestConfig =
            toml::from_str(toml_data_num).expect("Failed to deserialize TOML");
        assert_eq!(config_num.value, 987654321_u32);
    }
    #[test]
    fn test_deserialize_toml_u32_errors() {
        #[derive(serde::Deserialize, Debug)]
        struct TestConfig {
            #[serde(deserialize_with = "deserialize_prefixed_u32")]
            value: u32,
        }

        let toml_data_neg = r#"
            value = "-12345"
        "#;

        let result: Result<TestConfig, _> = toml::from_str(toml_data_neg);
        assert!(result.is_err(), "Expected error for negative value");

        let toml_data_float = r#"
            value = 12.34
        "#;

        let result_float: Result<TestConfig, _> = toml::from_str(toml_data_float);
        assert!(
            result_float.is_err(),
            "Expected error for floating point value"
        );

        let toml_data_overflow = r#"
            value = "0x1FFFFFFFF"
        "#;

        let result_overflow: Result<TestConfig, _> = toml::from_str(toml_data_overflow);
        assert!(
            result_overflow.is_err(),
            "Expected error for overflow value"
        );
    }

    #[test]
    fn test_deserialize_toml_u8() {
        #[derive(serde::Deserialize, Debug)]
        struct TestConfig {
            #[serde(deserialize_with = "deserialize_prefixed_u8")]
            value: u8,
        }
        let toml_data = r#"
            value = "0x1A"
        "#;
        let config: TestConfig = toml::from_str(toml_data).expect("Failed to deserialize TOML");
        assert_eq!(config.value, 0x1A_u8);

        let toml_data_bin = r#"
            value = "0b1101"
        "#;
        let config_bin: TestConfig =
            toml::from_str(toml_data_bin).expect("Failed to deserialize TOML");
        assert_eq!(config_bin.value, 13_u8);

        let toml_data_dec = r#"
            value = "123"
        "#;
        let config_dec: TestConfig =
            toml::from_str(toml_data_dec).expect("Failed to deserialize TOML");
        assert_eq!(config_dec.value, 123_u8);

        let toml_data_num = r#"
            value = 200
        "#;
        let config_num: TestConfig =
            toml::from_str(toml_data_num).expect("Failed to deserialize TOML");
        assert_eq!(config_num.value, 200_u8);
    }
    #[test]
    fn test_deserialize_toml_u8_errors() {
        #[derive(serde::Deserialize, Debug)]
        struct TestConfig {
            #[serde(deserialize_with = "deserialize_prefixed_u8")]
            value: u8,
        }

        let toml_data_neg = r#"
            value = "-123"
        "#;

        let result: Result<TestConfig, _> = toml::from_str(toml_data_neg);
        assert!(result.is_err(), "Expected error for negative value");

        let toml_data_float = r#"
            value = 12.34
        "#;

        let result_float: Result<TestConfig, _> = toml::from_str(toml_data_float);
        assert!(
            result_float.is_err(),
            "Expected error for floating point value"
        );

        let toml_data_overflow = r#"
            value = "0x1FF"
        "#;

        let result_overflow: Result<TestConfig, _> = toml::from_str(toml_data_overflow);
        assert!(
            result_overflow.is_err(),
            "Expected error for overflow value"
        );
    }
}
