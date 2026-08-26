# station-fare-processor

A small network of embedded kiosk nodes (fare tap-in/out, station display)
that share a consistent data model, inspired by transit systems where
individually modest components coordinate to behave as one system.

## Status
Early design phase — see /docs/adr for architecture decisions.

## Core problem
Not real-time safety control — the hard problem here is keeping kiosk
state consistent across nodes when events can arrive out of order,
duplicated, or during a network outage.
