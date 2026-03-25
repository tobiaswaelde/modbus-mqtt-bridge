# Concepts

The runtime consists of three main loops:

- MQTT event loop: maintains broker connection and receives `/set` writes
- Polling loop per source: reads configured Modbus points
- Write handling: validates and writes incoming command payloads

Each source runs independently so one slow endpoint does not block others.
