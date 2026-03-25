# Recipes

## Home Assistant

```yaml
mqtt:
  sensor:
    - name: "Boiler Temperature"
      state_topic: "modbus/example-device/telemetry/boiler_temp"
      unit_of_measurement: "°C"
      device_class: temperature
      state_class: measurement

  switch:
    - name: "Pump Enable"
      state_topic: "modbus/example-device/controls/pump_enable"
      command_topic: "modbus/example-device/controls/pump_enable/set"
      payload_on: "true"
      payload_off: "false"
```

## Node-RED

```text
[MQTT in] -> [Function: derive alarm] -> [Switch] -> [MQTT out]
```

Suggested topics:

- read from `modbus/example-device/telemetry/pressure`
- write to `modbus/example-device/controls/fan_speed/set`

## Mosquitto CLI

```bash
mosquitto_sub -h localhost -t 'modbus/+/+'
mosquitto_pub -h localhost -t 'modbus/example-device/controls/pump_enable/set' -m true
```
