# 5. Kiosks publish to "ent-ex/<kiosk_id>/taps"

Date: 2026-08-30

## Status
Accepted

## Context
MQTT requires that topics are setup for the different bits of data so that each subscriber knows where to find the data it's looking for.

## Decision
The enterance-exit kiosks will publish the TapEventData to "ent-ex/<kiosk_id>/taps"
 - en-ex to differentiate entrance/exit kiosks from balance/management kiosks
 - kiosk_id so that each kiosk will have it's own location to subscribe to to help scalability. the server can replace this part with a wild card to subscribe to all kiosks at once
 - taps to uniquely identify tap data in case later on there is more types of data that needs to be published by the kiosk

## Consequences
 - this should help organize the mqtt data
 - unique topics for each kiosk should help with scalability and maintainability
 - now that there is an agreed upon topic all componetns can work to use that topic in their logic: the dev-automations, the kiosk-entrance-exit, and the server