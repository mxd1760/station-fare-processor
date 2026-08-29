# 4. will be using rust unit testing

Date: 2026-08-29

## Status
Accepted

## Context
Being able to build up functionality and compliance from unit tests will help with tracking and maintaining both key features and 
edge cases in a way that helps overall software quality

## Decision
The built in unit testing framework that rust has will be used to define project requirements and confirm functionality for all 
required components of this project.

## Consequences
- Improved maintainability of each component
- bugs will require new tests cases to validate continued compliance
- all tests should be easy to run from the workspace
