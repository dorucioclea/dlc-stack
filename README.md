# dlc-stack

![build workflow](https://github.com/github/docs/actions/workflows/docker-build.yml/badge.svg)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

It is composed of multiple modules that work together to provide a seamless lending experience for DLC.Link stack.

## Modules

### Oracle

The `oracle` module is providing a numeric oracle implementation for bitcoin. (creating events / announcements / attestations).

### Oracle-discovery

The `oracle-discovery` module is responsible for discovering and registering oracles on the network. It provides an API for finding and connecting to available oracles.

## Wallet backedn
The wallet module is responsible for communicating with the dlc-manager and oracle. It provides an API for creating and managing loan transactions.

### Storage-API

The `storage-api `module provides an API for hiding storage operations. Currently, it is implemented to work with Postgres as the underlying storage engine.

### Clients

The `clients` module provies re-usable clients for the oracle / wallet / storage-api.

### IT

The it module provides basic integration tests using BDD (Behavior-Driven Development) with Cucumber.

## Build

```bash
make build
```

## License 

APM 2.0
