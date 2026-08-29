# 2. Basic Event Data

Date: 2026-08-29

## Status
Accepted

## Context
This helps build out a foundational structure to make further decisions

## Decision
The basic event will consist of 3 fields
 - kiosk_id : which kiosk the card is being tapped at (this also encodes entrance or exit because kiosks are one or the other)
 - card_id : the users card
 - timestamp : when the transaction was performed

## Consequences
 - kiosks will be required to be in or out
 - kiosk position tracking will be important in calculating fares
