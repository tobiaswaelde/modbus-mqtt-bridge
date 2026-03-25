# Troubleshooting

If data is not flowing as expected, check:

- MQTT broker reachability and credentials
- Modbus host, unit ID, register address, and function type
- exact topic match between config and publisher/subscriber
- point access mode before writing to `/set`
- logs for timeout/decode/payload errors

Quick isolation:

```bash
mosquitto_pub -h localhost -t 'modbus/example-device/controls/pump_enable/set' -m true
```
