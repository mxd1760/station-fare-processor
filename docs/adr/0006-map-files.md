# 6. Mapping kiosks to rail lines

Date: 2026-09-05

## Status
Accepted

## Context
Fare calculation is dependant on kiosk position, station/track layout, as well as other things like distances between stations, rail lines, fare calculateion modes, and more. 

## Decision
To input/store information about the kiosk layout and map layout the map.toml files will contiain a list of stations each with their own id's, names, and kiosks, as well as a list of connections from one station to another. further metadata like which line the station or connection belongs to, the geospacial coordinates of a given station, bidirectionality or lacktherof of a connection, or any other aspect that may end up being required by this file can be added to this format along with the schema for it stored in the dev-automations crate.

## Consequences
- metadata will change how files work slightly
- every railroad layout should more or less be able to be converted into this kind of file that contains enough data in conjuction with a fare-rules.toml file to calculate the fares appropriately
- kiosk layouts shoudl be relatively easy to change without much extra overhead beyond needing to run the dev-automation to solve for the fares again.
