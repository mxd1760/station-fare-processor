# 9. Account Creation

Date: 2026-09-19

## Status

Accepted

## Context

The Station Fare Processor (SFP) requires a mechanism for users to create and access transit accounts through a payment kiosk. Each user should have one SFP account associated with a transit card that can be presented at entry, exit, and payment kiosks.

The physical transit card should function primarily as an identifier for the user's SFP account rather than as a storage mechanism for account or payment information. The server is responsible for maintaining authoritative account state.

A user may also lose a transit card or otherwise need to stop using a particular card. The account and the card therefore need to be represented separately so that a card can be deactivated without deleting the associated account.

The initial system does not need to model a complete real-world customer identity system. However, the account creation process should establish some unique registration information that can be used to prevent multiple SFP accounts from being created for the same user.

## Decision

The payment kiosk will support two primary account interactions:

1. Creating a new account and transit card.
2. Accessing an existing account using a transit card.

Each SFP account will be associated with a single user and initially with one active transit card. The account and card will have separate identifiers.

Account creation will proceed approximately as follows:

1. The user selects account creation at a payment kiosk.
2. The kiosk collects the required registration information from the user.
3. The kiosk requests account creation from the SFP server.
4. The server determines whether the registration information can be used to create a new account.
5. The server creates the account and generates a unique transit card identifier.
6. The server associates the new card with the account.
7. The kiosk receives the newly created account/card information.
8. The kiosk produces a physical card containing or representing the assigned card identifier.
9. The newly created account is considered logged in for the current kiosk session.

The exact form of the registration identifier is intentionally left open to implementation. The initial implementation may use an email address, telephone number, or another unique user-provided identifier. Payment credentials such as bank or credit card information will not be used as the primary means of identifying a user.

When an existing card is presented, the kiosk will send the card identifier to the server. The server will determine the associated account and return the account information that the kiosk is permitted to display.

A card may be deactivated independently of its account. This allows a lost or compromised card to be invalidated without deleting the user's account or account history. Replacement-card functionality is not required for the initial implementation, but the data model must not prevent it from being added later.

The payment kiosk will not independently maintain authoritative account state. Account balance, card validity, and other persistent account information will be determined by the server.

## Consequences

* A transit card acts as an account credential/identifier rather than as an account database.
* Losing a card does not re
