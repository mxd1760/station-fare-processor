# 1. Core problem is data consistency across nodes

Date: 2026-08-26

## Status
Accepted

## Context
This project is about handling many users getting on and off of transit at various points along a path. 
They should pay for the distance they were moved. This problem requires multiple synchronized nodes across 
the network and validating the data getting processed by them to form the complete and correct transactions 
is paramount to this projects success.

## Decision
The core engineering problem for this project is: keeping kiosk/ledger
state consistent across networked nodes under duplicate, out-of-order,
or delayed events (idempotency, eventual consistency).

## Consequences
- Hardware skills (I2C card reader, SPI display, UART/WiFi networking,
  RTOS for local concurrency) are required.
- Future ADRs needed: how nodes reconcile with a central ledger, what
  happens during network partition, event schema design.
