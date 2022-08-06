# MLB StatsAPI
Rust wrapper for interacting with the MLB Stats API - https://statsapi.mlb.com

## Copyright Notice
This repository and its author are not affiliated with the MLB in any way. Use of MLB data is subject to the notice posted at http://gdx.mlb.com/components/copyright.txt.

## A few things to note
For the time being, you'll need Rust's package manager - cargo - installed in order to install this properly

I am planning to develop this as an extension for Python using [pyo3](https://github.com/PyO3/pyo3) in addition to a Rust crate.

This library is very much in its early stages of development. There are only a few functions that have been written as of yet.

## Getting Started
### 1. Clone the repository
```bash
$ git clone https://github.com/joerex1418/mlbapi.git
```
### 2. Change Directory
```bash
$ cd mlbapi
```
### 3. Install the binary
```bash
$ cargo install --path .
```

## Example Usage
```bash
# Get basic information for a player given their official MLB Advanced Media ID
$ mlb player 547989
```
```bash
# Get stats for a player (Jose Abreu) by specifying a stat group after the `--stats` flag
$ mlb player 547989 --stats hitting
```


