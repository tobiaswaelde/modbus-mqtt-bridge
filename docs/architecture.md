# Architecture

The bridge sits between Modbus devices and MQTT consumers.

```text
┌──────────────────┐      Modbus TCP       ┌────────────────────────┐
│ PLC / inverter / │  <----------------->  │ Modbus MQTT Bridge     │
│ meter / sensor   │                      │  - polls points         │
└──────────────────┘                      │  - validates writes     │
                                           │  - publishes MQTT state │
                                           └───────────┬────────────┘
                                                       │ MQTT
                                                       v
                                           ┌────────────────────────┐
                                           │ MQTT broker            │
                                           └───────────┬────────────┘
                                                       │
                         ┌─────────────────────────────┼─────────────────────────────┐
                         v                             v                             v
                 Home Assistant                 Node-RED flows                 Mosquitto tools
```
