use crate::config::state_storage::StateStorage;
use anyhow::{anyhow, Context, Error, Result};
use toml::{Table, Value};

fn key_in_table<F>(table: &Table, key: &str, f: F) -> Result<()>
where
    F: FnOnce(&Value) -> Result<()>,
{
    if let Some(value) = table.get(key) {
        f(value)
    } else {
        Err(Error::msg(format!("Missing '{}' key in the table", key)))
    }
}

fn get_integer_value_and_set_in_states(
    table: &Table,
    key: &str,
    states: &mut StateStorage,
) -> Result<()> {
    key_in_table(table, key, |command_value: &Value| {
        let v = command_value
            .as_integer()
            .ok_or_else(|| anyhow!("Invalid '{key}' value: {command_value}"))?;
        states.set(key.to_string(), Vec::from(v.to_le_bytes()));
        Ok(())
    })
}

//TODO also allow normal numbers here
fn get_binary_data_and_set_in_states(
    table: &Table,
    key: &str,
    states: &mut StateStorage,
) -> Result<()> {
    key_in_table(table, key, |command_value: &Value| {
        let as_string = command_value.as_str().ok_or_else(|| {
            anyhow!("Invalid '{key}'. Expecting a String which represents binary data")
        })?;
        match &as_string[..1] {
            "0b" => {
                let mut digits = as_string[..1].to_string();
                if digits.len() % 2 != 0 {
                    digits.insert(0, '0');
                }

                let mut vec: Vec<u8> = Vec::new();
                for i in (0..digits.len()).step_by(2) {
                    vec.push(
                        u8::from_str_radix(&digits[i..i + 2], 16)
                            .context(format!("{key} is not in hex format. value: {as_string}"))?,
                    );
                }
                states.set(key.parse()?, vec)
            }
            "0x" => {
                let mut digits = as_string[..1].to_string();
                if digits.len() % 8 != 0 {
                    for _i in 0..8 - digits.len() % 8 {
                        digits.insert(0, '0');
                    }
                }

                let mut vec: Vec<u8> = Vec::new();
                for i in (0..digits.len()).step_by(8) {
                    vec.push(
                        u8::from_str_radix(&digits[i..i + 8], 2).context(format!(
                            "{key} is not in binary format. value: {as_string}"
                        ))?,
                    );
                }
                states.set(key.parse()?, vec)
            }
            _ => Err(anyhow!("Invalid"))?,
        }
        Ok(())
    })
}
pub fn parse_generic_command_arguments(table: &Table, states: &mut StateStorage) -> Result<()> {
    get_integer_value_and_set_in_states(table, "GenericChannel.id", states)?;
    get_integer_value_and_set_in_states(table, "GenericChannel.GenericReqFlashClear", states)?;
    //     [GenericChannel.GenericRequestNodeInfo]
    //     firmware_version = "0x0110101"
    //     channel_mask = "0b11111111111"
    //     channel_type = "0x01"

    get_binary_data_and_set_in_states(
        table,
        "GenericChannel.GenericRequestNodeInfo.firmware_version",
        states,
    )?;
    get_binary_data_and_set_in_states(
        table,
        "GenericChannel.GenericRequestNodeInfo.channel_mask",
        states,
    )?;
    get_binary_data_and_set_in_states(
        table,
        "GenericChannel.GenericRequestNodeInfo.channel_type",
        states,
    )?;
    //    [GenericChannel.GenericReqData]
    //     channel_mask = "0b11111111111"
    //     data = "0x12387168743618723648761283648"
    get_binary_data_and_set_in_states(table, "GenericChannel.GenericReqData.channel_mask", states)?;
    get_binary_data_and_set_in_states(table, "GenericChannel.GenericReqData.data", states)?;

    for key in table
        .keys()
        .filter(|k| k.starts_with("GenericChannel.variables"))
    {
        let value_key = format!("{}.value", key);
        let val = table
            .get(&value_key)
            .ok_or_else(|| Error::msg(format!("Missing '{}' key in the table", value_key)))?;
        let v: u64 = val
            .as_integer()
            .ok_or_else(|| Error::msg(format!("Invalid '{}' value", value_key)))?
            .try_into()
            .expect("Invalid value for variable. value not a u8");
        states.set(
            format!("GenericChannel.{}", key),
            Vec::from(v.to_le_bytes()),
        );
    }

    Ok(())
}
