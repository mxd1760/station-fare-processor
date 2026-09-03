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

## process
first a map is manually entered into one of the dev automation tools. this is where the properly ided kiosks will be configured and fares between any two kiosks will be calculated and stored in a few tables that the server will later use.

once the fare data is loaded the kiosks and server can start working. there are two broad types of kiosk. the entrance-exit kiosk and the payment kiosk. at a payment kiosk users can add money to a card (the data for this transaction will be stored on the server), this card can then be used at the entrance-exit kisoks to enter and exit the train stations. each entrance and exit event will send information to the server which will match the events and process charges to the card based on the trips taken and the fare data tables used prior.

## external utilities
the map images for this project were gathered from https://openrailwaymap.app 
