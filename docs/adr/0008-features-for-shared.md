# 8. Shared will be broken up by features

Date: 2026-09-05

## Status
Accepted

## Context
The shared crate is meant to allow different other crates to share information. this is importnat to let the different creates share models of how the program should run, but it also helps with keeping the no-std components from depending on std features.

## Decision
Using features on the shared crate will help ensure that only the crates that need access to a given feature will have access to it helping make each crate and espeically the no-std crates lighter weight without compromizing the ability for std creates to share code that uses std.

## Consequences
- smaller shared size when not needed
- std crates can share std code without breaking no-std crates
