use anyhow::{Result, bail};
use serde_json::{Number, Value};

use crate::config::{ByteOrder, DataType, Encoding, PointConfig, RegisterKind, WordOrder};

// Use the explicit register count when present, otherwise infer the usual width from the data type.
pub fn register_count(point: &PointConfig) -> u16 {
    point.count.unwrap_or_else(|| match point.data_type {
        DataType::Bool => 1,
        DataType::U16 | DataType::I16 => 1,
        DataType::U32 | DataType::I32 | DataType::F32 => 2,
        DataType::String => 1,
        DataType::RawU16 => 1,
    })
}

pub fn decode_point(
    point: &PointConfig,
    bits: Option<&[bool]>,
    registers: Option<&[u16]>,
) -> Result<Value> {
    let raw = match point.kind {
        RegisterKind::Coil | RegisterKind::DiscreteInput => {
            let values = bits.ok_or_else(|| anyhow::anyhow!("missing coil data"))?;
            let value = values
                .first()
                .copied()
                .ok_or_else(|| anyhow::anyhow!("empty coil response"))?;
            Value::Bool(value)
        }
        RegisterKind::Holding | RegisterKind::Input => {
            let words = registers.ok_or_else(|| anyhow::anyhow!("missing register data"))?;
            decode_registers(point, words)?
        }
    };

    apply_numeric_transform(raw, point.scale, point.offset)
}

pub fn encode_write_payload(point: &PointConfig, value: &Value) -> Result<EncodedWrite> {
    match point.kind {
        RegisterKind::Coil => Ok(EncodedWrite::Coil(coerce_bool(value)?)),
        RegisterKind::DiscreteInput => bail!("cannot write discrete inputs"),
        RegisterKind::Holding => encode_holding_write(point, value),
        RegisterKind::Input => bail!("cannot write input registers"),
    }
}

pub enum EncodedWrite {
    Coil(bool),
    Registers(Vec<u16>),
}

fn decode_registers(point: &PointConfig, registers: &[u16]) -> Result<Value> {
    let count = register_count(point) as usize;
    if registers.len() < count {
        bail!(
            "register response too short for point '{}' (expected {}, got {})",
            point.name,
            count,
            registers.len()
        );
    }

    let slice = &registers[..count];

    match point.data_type {
        DataType::Bool => Ok(Value::Bool(slice[0] != 0)),
        DataType::U16 => Ok(Value::Number(Number::from(slice[0]))),
        DataType::I16 => Ok(Value::Number(Number::from((slice[0] as i16) as i64))),
        DataType::U32 => Ok(Value::Number(Number::from(
            join_u32(slice, point.encoding) as u64
        ))),
        DataType::I32 => Ok(Value::Number(Number::from(
            join_u32(slice, point.encoding) as i32 as i64,
        ))),
        DataType::F32 => {
            let bits = join_u32(slice, point.encoding);
            let value = f32::from_bits(bits) as f64;
            let num =
                Number::from_f64(value).ok_or_else(|| anyhow::anyhow!("invalid f32 value"))?;
            Ok(Value::Number(num))
        }
        DataType::String => {
            let bytes = registers_to_bytes(slice, point.encoding);
            let value = String::from_utf8_lossy(&bytes)
                .trim_end_matches('\0')
                .trim_end()
                .to_string();
            Ok(Value::String(value))
        }
        DataType::RawU16 => Ok(Value::Array(
            slice
                .iter()
                .copied()
                .map(|word| Value::Number(Number::from(word)))
                .collect(),
        )),
    }
}

fn encode_holding_write(point: &PointConfig, value: &Value) -> Result<EncodedWrite> {
    let registers = match point.data_type {
        DataType::Bool => vec![u16::from(coerce_bool(value)?)],
        DataType::U16 => vec![coerce_u64(value)? as u16],
        DataType::I16 => vec![coerce_i64(value)? as i16 as u16],
        DataType::U32 => split_u32(coerce_u64(value)? as u32, point.encoding),
        DataType::I32 => split_u32(coerce_i64(value)? as i32 as u32, point.encoding),
        DataType::F32 => split_u32((coerce_f64(value)? as f32).to_bits(), point.encoding),
        DataType::String => {
            let text = value
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("expected string payload"))?;
            let count = register_count(point) as usize;
            bytes_to_registers(text.as_bytes(), count, point.encoding)
        }
        DataType::RawU16 => {
            let array = value
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("expected array of integers"))?;
            array
                .iter()
                .map(|entry| Ok(coerce_u64(entry)? as u16))
                .collect::<Result<Vec<_>>>()?
        }
    };

    Ok(EncodedWrite::Registers(registers))
}

fn join_u32(words: &[u16], encoding: Encoding) -> u32 {
    let ordered = order_words(words, encoding.word_order);
    // Normalize word order first, then apply byte order inside each 16-bit register.
    let bytes = registers_to_bytes(
        &ordered,
        Encoding {
            byte_order: encoding.byte_order,
            word_order: WordOrder::Big,
        },
    );

    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn split_u32(value: u32, encoding: Encoding) -> Vec<u16> {
    let bytes = value.to_be_bytes();
    let regs = bytes_to_registers(
        &bytes,
        2,
        Encoding {
            byte_order: encoding.byte_order,
            word_order: WordOrder::Big,
        },
    );
    order_words(&regs, encoding.word_order)
}

fn registers_to_bytes(words: &[u16], encoding: Encoding) -> Vec<u8> {
    let ordered = order_words(words, encoding.word_order);
    let mut bytes = Vec::with_capacity(ordered.len() * 2);
    for word in ordered {
        let pair = match encoding.byte_order {
            ByteOrder::Big => word.to_be_bytes(),
            ByteOrder::Little => word.to_le_bytes(),
        };
        bytes.extend_from_slice(&pair);
    }
    bytes
}

fn bytes_to_registers(bytes: &[u8], register_count: usize, encoding: Encoding) -> Vec<u16> {
    let mut padded = bytes.to_vec();
    // Strings and raw payloads are padded to the configured register width before writing.
    padded.resize(register_count * 2, 0);

    let mut regs = Vec::with_capacity(register_count);
    for chunk in padded.chunks(2).take(register_count) {
        let word = match encoding.byte_order {
            ByteOrder::Big => u16::from_be_bytes([chunk[0], chunk[1]]),
            ByteOrder::Little => u16::from_le_bytes([chunk[0], chunk[1]]),
        };
        regs.push(word);
    }

    order_words(&regs, encoding.word_order)
}

fn order_words(words: &[u16], word_order: WordOrder) -> Vec<u16> {
    let mut ordered = words.to_vec();
    if matches!(word_order, WordOrder::Little) {
        ordered.reverse();
    }
    ordered
}

fn apply_numeric_transform(value: Value, scale: Option<f64>, offset: Option<f64>) -> Result<Value> {
    if scale.is_none() && offset.is_none() {
        return Ok(value);
    }

    match value {
        Value::Number(number) => {
            let mut result = number
                .as_f64()
                .ok_or_else(|| anyhow::anyhow!("failed to convert number to f64"))?;
            if let Some(scale) = scale {
                result *= scale;
            }
            if let Some(offset) = offset {
                result += offset;
            }
            let transformed = Number::from_f64(result)
                .ok_or_else(|| anyhow::anyhow!("invalid transformed numeric value"))?;
            Ok(Value::Number(transformed))
        }
        other => Ok(other),
    }
}

fn coerce_bool(value: &Value) -> Result<bool> {
    match value {
        Value::Bool(value) => Ok(*value),
        Value::Number(value) => Ok(value.as_i64().unwrap_or_default() != 0),
        Value::String(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            match normalized.as_str() {
                "1" | "true" | "on" | "yes" => Ok(true),
                "0" | "false" | "off" | "no" => Ok(false),
                _ => bail!("unsupported boolean string '{}'", value),
            }
        }
        _ => bail!("cannot coerce payload to bool"),
    }
}

fn coerce_u64(value: &Value) -> Result<u64> {
    if let Some(number) = value.as_u64() {
        return Ok(number);
    }

    if let Some(number) = value.as_i64() {
        return Ok(number as u64);
    }

    if let Some(text) = value.as_str() {
        return text.parse::<u64>().map_err(Into::into);
    }

    bail!("cannot coerce payload to u64")
}

fn coerce_i64(value: &Value) -> Result<i64> {
    if let Some(number) = value.as_i64() {
        return Ok(number);
    }

    if let Some(number) = value.as_u64() {
        return Ok(number as i64);
    }

    if let Some(text) = value.as_str() {
        return text.parse::<i64>().map_err(Into::into);
    }

    bail!("cannot coerce payload to i64")
}

fn coerce_f64(value: &Value) -> Result<f64> {
    if let Some(number) = value.as_f64() {
        return Ok(number);
    }

    if let Some(text) = value.as_str() {
        return text.parse::<f64>().map_err(Into::into);
    }

    bail!("cannot coerce payload to f64")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Access, PointConfig, RegisterKind};

    fn point(data_type: DataType, encoding: Encoding) -> PointConfig {
        PointConfig {
            name: "demo".into(),
            topic: "demo".into(),
            address: 1,
            kind: RegisterKind::Holding,
            data_type,
            access: Access::ReadWrite,
            count: None,
            encoding,
            scale: None,
            offset: None,
            retain: None,
        }
    }

    #[test]
    fn decodes_big_endian_f32() {
        let point = point(DataType::F32, Encoding::default());
        let value = decode_point(&point, None, Some(&[0x4000, 0x0000])).unwrap();
        assert_eq!(value, Value::Number(Number::from_f64(2.0).unwrap()));
    }

    #[test]
    fn encodes_little_word_order_u32() {
        let point = point(
            DataType::U32,
            Encoding {
                byte_order: ByteOrder::Big,
                word_order: WordOrder::Little,
            },
        );

        match encode_write_payload(&point, &Value::Number(Number::from(0x11223344u64))).unwrap() {
            EncodedWrite::Registers(words) => assert_eq!(words, vec![0x3344, 0x1122]),
            EncodedWrite::Coil(_) => panic!("expected register write"),
        }
    }
}
