# 3. Using MQTT server protocol

Date: 2026-08-29

## Status
Accepted

## Context
This project is dependent on a large fleet of kiosks being able to have a shared data model 
of what's going on through connection to a server. This server will need to be optimized for 
synchronizing these kinds of devices

## Decision
We will use MQTT as the server protocol because of its common usage in IOT and embedded projects
where large numbers of devices with fluctuation reliability need to be simultaneously communicated 
with.

## Consequences
- low server overhead for each connected device
- good at dealing with unreliable devices and confirming messages are received.
