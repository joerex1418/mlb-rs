# mlbapi
Rust wrapper for interacting with the MLB Stats API - https://statsapi.mlb.com

## Copyright Notice
This repository and its author are not affiliated with the MLB in any way. Use of MLB data is subject to the notice posted at http://gdx.mlb.com/components/copyright.txt.

## A few things to note
I am planning to develop this as an extension for Python using [pyo3](https://github.com/PyO3/pyo3).

At the moment, this library is very much in its early stages of development. There are only a few functions that have been written as of yet.

## Working Functions
These functions will return objects as "Optionals" that will need to be unwrapped
```rust
// Returns a Rust object containing a person's biographical data
get_person(person_id:usize) -> Option<Person>

// Returns Rust object containing basic team info
get_team(team_id:usize) -> Option<Team>

// Returns Schedule JSON response as a Rust object
get_schedule(date:Option<String>) -> Option<ScheduleResponse>

// Returns Roster JSON response as a Rust object
get_roster(team_id:usize) -> Option<RosterResponse>
```

