# 7. Map.toml and fare_rules.toml get compiled by dev-automations into .fares files

Date: 2026-09-05

## Status
Accepted

## Context
Both fares and kiosk positions are subject to change so being able to change them in configuration files and then compile them into new fare files used by the server is an important process.

## Decision
A map.toml will store information about the track/station/kiosks, a fare_rules.toml will store information about pricing strategy and actual costs. these two files together will be compiled by the "dev-automations solve-fares" command into a .fares file that can be used by the server to process fares accurately and at an appropriate price.

## Consequences
- changes to layout or rules will not require changes to code unless it isn't modeled by one of the available modes.
- fares being precompiled will make processing fares more or less isntant for the server since the long calculation would have already been completed by the dev-automations task
