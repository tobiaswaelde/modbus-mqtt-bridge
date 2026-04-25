use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use modbus_mqtt_bridge::{
    config::{Access, AppConfig, DataType, Encoding, PointConfig, RegisterKind},
    modbus_codec::{EncodedWrite, decode_point, encode_write_payload, register_count},
};
use serde_json::{Number, Value, json};

fn unique_temp_file(name: &str, ext: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("modbus_mqtt_bridge_{name}_{stamp}.{ext}"))
}

#[test]
fn loads_yaml_config_and_validates() {
    let path = unique_temp_file("config", "yml");
    let raw = r#"
mqtt:
  host: localhost
sources:
  - id: source-a
    host: 127.0.0.1
    unit_id: 1
    points:
      - name: p1
        topic: t1
        address: 0
        kind: holding
        data_type: u16
"#;
    fs::write(&path, raw).expect("write config");
    let config = AppConfig::load(&path).expect("load config");
    config.validate().expect("validate config");
    assert_eq!(config.sources.len(), 1);
    let _ = fs::remove_file(path);
}

#[test]
fn loads_json_config_and_validates() {
    let path = unique_temp_file("config", "json");
    let raw = r#"{
  "mqtt": { "host": "localhost" },
  "sources": [
    {
      "id": "source-a",
      "host": "127.0.0.1",
      "unit_id": 1,
      "points": [
        {
          "name": "p1",
          "topic": "t1",
          "address": 0,
          "kind": "coil",
          "data_type": "bool"
        }
      ]
    }
  ]
}"#;
    fs::write(&path, raw).expect("write config");
    let config = AppConfig::load(&path).expect("load config");
    config.validate().expect("validate config");
    assert_eq!(config.mqtt.host, "localhost");
    let _ = fs::remove_file(path);
}

#[test]
fn validate_rejects_duplicate_source_ids() {
    let path = unique_temp_file("config_invalid_duplicate_source", "yml");
    let raw = r#"
mqtt:
  host: localhost
sources:
  - id: source-a
    host: 127.0.0.1
    unit_id: 1
    points:
      - name: p1
        topic: t1
        address: 0
        kind: holding
        data_type: u16
  - id: source-a
    host: 127.0.0.2
    unit_id: 1
    points:
      - name: p2
        topic: t2
        address: 1
        kind: holding
        data_type: u16
"#;
    fs::write(&path, raw).expect("write config");
    let config = AppConfig::load(&path).expect("load config");
    let error = config.validate().expect_err("validation should fail");
    assert!(error.to_string().contains("duplicate source id"));
    let _ = fs::remove_file(path);
}

#[test]
fn validate_rejects_duplicate_point_topics_per_source() {
    let path = unique_temp_file("config_invalid_duplicate_topic", "yml");
    let raw = r#"
mqtt:
  host: localhost
sources:
  - id: source-a
    host: 127.0.0.1
    unit_id: 1
    points:
      - name: p1
        topic: telemetry/temp
        address: 0
        kind: holding
        data_type: u16
      - name: p2
        topic: telemetry/temp
        address: 1
        kind: holding
        data_type: u16
"#;
    fs::write(&path, raw).expect("write config");
    let config = AppConfig::load(&path).expect("load config");
    let error = config.validate().expect_err("validation should fail");
    assert!(error.to_string().contains("duplicate point topic"));
    let _ = fs::remove_file(path);
}

#[test]
fn validate_rejects_wildcard_topics() {
    let path = unique_temp_file("config_invalid_wildcard_topic", "yml");
    let raw = r#"
mqtt:
  host: localhost
sources:
  - id: source-a
    host: 127.0.0.1
    unit_id: 1
    points:
      - name: p1
        topic: telemetry/#
        address: 0
        kind: holding
        data_type: u16
"#;
    fs::write(&path, raw).expect("write config");
    let config = AppConfig::load(&path).expect("load config");
    let error = config.validate().expect_err("validation should fail");
    assert!(
        error
            .to_string()
            .contains("must not contain MQTT wildcards")
    );
    let _ = fs::remove_file(path);
}

#[test]
fn validate_rejects_set_suffix_in_point_topic() {
    let path = unique_temp_file("config_invalid_set_suffix", "yml");
    let raw = r#"
mqtt:
  host: localhost
sources:
  - id: source-a
    host: 127.0.0.1
    unit_id: 1
    points:
      - name: p1
        topic: telemetry/temp/set
        address: 0
        kind: holding
        data_type: u16
"#;
    fs::write(&path, raw).expect("write config");
    let config = AppConfig::load(&path).expect("load config");
    let error = config.validate().expect_err("validation should fail");
    assert!(error.to_string().contains("must not end with '/set'"));
    let _ = fs::remove_file(path);
}

#[test]
fn validate_rejects_zero_poll_interval() {
    let path = unique_temp_file("config_invalid_poll_interval", "yml");
    let raw = r#"
mqtt:
  host: localhost
sources:
  - id: source-a
    host: 127.0.0.1
    unit_id: 1
    poll_interval_ms: 0
    points:
      - name: p1
        topic: t1
        address: 0
        kind: holding
        data_type: u16
"#;
    fs::write(&path, raw).expect("write config");
    let config = AppConfig::load(&path).expect("load config");
    let error = config.validate().expect_err("validation should fail");
    assert!(
        error
            .to_string()
            .contains("poll_interval_ms must be greater than 0")
    );
    let _ = fs::remove_file(path);
}

#[test]
fn register_count_defaults_follow_data_type() {
    let point = PointConfig {
        name: "demo".into(),
        topic: "demo".into(),
        address: 0,
        kind: RegisterKind::Holding,
        data_type: DataType::F32,
        access: Access::ReadWrite,
        count: None,
        encoding: Encoding::default(),
        scale: None,
        offset: None,
        retain: None,
    };
    assert_eq!(register_count(&point), 2);
}

#[test]
fn decode_and_encode_f32_roundtrip_shape() {
    let point = PointConfig {
        name: "temperature".into(),
        topic: "telemetry/temperature".into(),
        address: 10,
        kind: RegisterKind::Holding,
        data_type: DataType::F32,
        access: Access::ReadWrite,
        count: None,
        encoding: Encoding::default(),
        scale: None,
        offset: None,
        retain: None,
    };

    let decoded = decode_point(&point, None, Some(&[0x4120, 0x0000])).expect("decode f32");
    assert_eq!(decoded, Value::Number(Number::from_f64(10.0).unwrap()));

    match encode_write_payload(&point, &json!(10.0)).expect("encode f32") {
        EncodedWrite::Registers(words) => assert_eq!(words.len(), 2),
        EncodedWrite::Coil(_) => panic!("expected register write"),
    }
}
