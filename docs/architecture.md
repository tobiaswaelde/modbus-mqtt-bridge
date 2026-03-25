# Architecture

The bridge sits between Modbus TCP devices and MQTT consumers.

```mermaid
flowchart LR
  subgraph Field["Field Layer"]
    DEV["Modbus Device\n(PLC / Inverter / Meter)"]
  end

  subgraph Bridge["Bridge Runtime"]
    BRIDGE["Modbus MQTT Bridge\n- Polls configured points\n- Publishes state topics\n- Handles /set writes"]
  end

  subgraph Messaging["Messaging Layer"]
    BROKER["MQTT Broker\n(Mosquitto / EMQX / HiveMQ)"]
  end

  subgraph Consumers["Consumer Layer"]
    HA["Home Assistant"]
    NR["Node-RED"]
    CLI["CLI / Scripts"]
  end

  DEV <--> |Modbus TCP| BRIDGE
  BRIDGE <--> |MQTT| BROKER
  BROKER --> HA
  BROKER --> NR
  BROKER --> CLI
```

## Data flow

1. The bridge polls Modbus points from one or more sources.
2. Values are decoded and published to MQTT state topics.
3. Write commands are received on matching `/set` topics.
4. The bridge encodes and writes values back to Modbus.

## Design goals

- isolate each Modbus source in its own poll loop
- keep topic structure stable and predictable
- recover gracefully from transient broker/device failures
