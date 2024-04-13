# agscheduler-cli

[![test](https://github.com/agscheduler/agscheduler-cli/actions/workflows/test.yml/badge.svg)](https://github.com/agscheduler/agscheduler-cli/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/AGScheduler/agscheduler-cli/graph/badge.svg?token=2KUVNBKH9K)](https://codecov.io/gh/AGScheduler/agscheduler-cli)
[![Crates.io](https://img.shields.io/crates/v/agscheduler-cli.svg)](https://crates.io/crates/agscheduler-cli)
[![license](https://img.shields.io/github/license/agscheduler/agscheduler)](https://github.com/agscheduler/agscheduler/blob/main/LICENSE)

> Command line interface for AGScheduler

## Installation

```bash
cargo install agscheduler-cli
```

## Usage

```bash
$ agscheduler-cli -h            
Command line interface for AGScheduler

Usage: agscheduler-cli [OPTIONS]

Options:
  -e, --endpoint <ENDPOINT>  AGScheduler HTTP endpoint [default: http://127.0.0.1:36370]
  -h, --help                 Print help
  -V, --version              Print version


$ agscheduler-cli   
Connecting to `http://127.0.0.1:36370`...
? Select your operation › [Page 1/3] 
  Get Info
  Get Funcs
  Get Job
❯ Get All Jobs

✔ Select your operation · Get All Jobs
+------------------+-------+----------+---------------------+---------------------+---------+
| ID               | Name  | Type     | LastRunTime         | NextRunTime         | Status  |
+===========================================================================================+
| 8088b567cc3a4345 | myJob | interval | 2024-02-18 00:17:28 | 2024-02-18 00:18:28 | running |
+------------------+-------+----------+---------------------+---------------------+---------+
```

## Development

```bash
# Clone code
git clone git@github.com:agscheduler/agscheduler-cli.git

# Working directory
cd agscheduler-cli

# Install dependencies
make install

# Run check
make check-all
```
