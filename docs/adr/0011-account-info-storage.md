# 11. Account Information Storage on Server

Date: 2026-09-19

## Status

Accepted

## Context

SFP requires persistent server-side state for transit accounts, transit cards, balances, payment methods, and financial transactions.

The payment kiosk should not be responsible for maintaining authoritative account state. Kiosks may display account information and request operations, but the server must determine the current state of an account.

The system will also eventually need to associate transit journeys and fare payments with accounts so that the fare processor can charge the correct user and maintain a transaction history.

The data model should therefore distinguish between a user/account, their transit card, their payment method, and individual financial transactions.

## Decision

The SFP server will maintain the authoritative state for accounts and their associated transit cards.

An account will conceptually contain:

* A unique account identifier.
* A unique user registration identifier.
* An associated transit card.
* A current account balance.
* Account status information.

A transit card will conceptually contain:

* A unique card identifier.
* The associated account identifier.
* A card status indicating whether the card may currently be used.

The card identifier will not contain or encode financial information.

Payment methods will be represented separately from the transit card. A payment method will identify the mechanism through which funds can be added to an SFP account. The initial implementation may use a simulated payment method rather than an actual external banking service.

Financial operations will be recorded as transactions rather than simply modifying the account balance without history. A transaction should contain enough information to identify:

* The affected account.
* The transaction type.
* The amount.
* The resulting operation status.
* The relevant timestamp or event time.
* A unique transaction identifier.

Transit fare payments will eventually be represented as transactions associated with the relevant account and journey.

The server will be the authoritative source for account balances, card validity, and transaction state. A kiosk may cache or display information temporarily, but such information will not supersede the server's state.

Card deactivation will not delete the associated account or transaction history.

## Consequences

* Account state has a clear authoritative owner.
* Transit cards can be invalidated independently of accounts.
* Financial history can be inspected independently of the current account balance.
* Payment processing can eventually be connected to the fare-processing system without requiring the physical card to contain financial state.
* A simulated payment provider can be replaced with a real provider later without fundamentally changing the account/card relationship.
* The data model leaves room for future card replacement and potentially multiple cards per account.
* Server-side persistence becomes an important part of the eventual payment and fare-processing implementation.

## Out of Scope

The initial implementation does not require:

* Real banking integration.
* Automatic payments or automatic balance replenishment.
* Multiple active cards per account.
* A complete customer identity-verification system.
* A production-grade security model.
* Detailed accounting or regulatory compliance.
* A finalized database technology or schema.
