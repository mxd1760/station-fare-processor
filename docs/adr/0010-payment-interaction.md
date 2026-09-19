# 10. Payment Interaction

Date: 2026-09-19

## Status

Accepted

## Context

SFP payment kiosks need to communicate with the SFP server to create accounts, authenticate transit cards, retrieve account information, and request account payments.

MQTT has already been selected as the communication transport for SFP clients and the server. MQTT provides message delivery but does not by itself define the meaning or sequence of SFP operations.

The application therefore requires a protocol defining the operations and state transitions that occur between a payment kiosk and the SFP server.

The same conceptual protocol should be usable by both physical kiosks and simulated clients so that the server does not need separate business logic for each type of client.

## Decision

SFP will define an application-level kiosk/server protocol carried over MQTT.

The protocol will define operations based on the action being requested rather than exposing the server's internal implementation. At minimum, the initial protocol will support:

* Account creation.
* Card authentication.
* Account information retrieval.
* Account balance payment/replenishment.
* Results for successful and unsuccessful operations.

Conceptually, the interaction will follow patterns such as:

### Account Creation

```text
Payment Kiosk
    |
    | account creation request
    v
SFP Server
    |
    | account created + card identifier
    v
Payment Kiosk
```

### Card Authentication

```text
Payment Kiosk
    |
    | card identifier
    v
SFP Server
    |
    | authentication result + account information
    v
Payment Kiosk
```

### Account Payment

```text
Payment Kiosk
    |
    | payment request
    v
SFP Server
    |
    | payment processing
    |
    | payment result
    v
Payment Kiosk
```

The kiosk will request financial operations but will not directly modify the server's account balance.

The server will validate requests, perform the requested operation, update persistent state, and return the result to the kiosk.

MQTT topics, message serialization, request identifiers, error representation, acknowledgement behavior, and retry semantics will be defined as part of the implementation of this protocol. These details should remain separate from the higher-level decision that MQTT is the transport mechanism.

The protocol should also be designed so that a future physical transit kiosk and the visual simulator can use the same server-facing operations where appropriate.

## Consequences

* Payment kiosks remain relatively thin clients.
* Financial and account state remains authoritative on the server.
* Physical and simulated kiosks can exercise the same server functionality.
* MQTT can be replaced or extended in the future without changing the conceptual kiosk/server operations.
* Requests can be tested independently of the physical kiosk hardware.
* The protocol will eventually need explicit handling for failures such as invalid cards, insufficient balance, invalid account creation information, and failed payment operations.

## Out of Scope

The initial protocol does not define:

* Real-world banking APIs.
* Automatic balance replenishment.
* Physical card communication standards.
* The exact MQTT topic hierarchy.
* The exact serialization format.
* A complete security/authentication system.
* Transit entry/exit transactions, except where they establish requirements for shared account/card concepts.
