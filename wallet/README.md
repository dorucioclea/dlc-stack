# DLC-link-backend

A test backend to:
* generating offer messages for enumerated and numerical contracts
* receiving accept messages and returning sign messages from them

Run with:
```bash
cargo run
```

Run against dev test environment:

```bash
BTC_RPC_URL="electrs-btc2.dlc.link:18443/wallet/alice" RPC_USER="devnet2" RPC_PASS="devnet2" ORACLE_URL="https://dev-oracle.dlc.link/oracle" RUST_LOG=debug cargo run
```

Docker Compose example:

- go into docker folder and create a .env like this:

```
ORACLE_URL="http://dev-oracle.dlc.link" 
BTC_RPC_URL="electrs-btc2.dlc.link:18443/wallet/alice" 
RPC_USER="devnet2" 
RPC_PASS="devnet2"
USE_SLED=true
DOCKER_REGISTRY_PREFIX=public.ecr.aws/dlc-link/
USE_POSTGRES=false
INIT_DB=false
DATABASE_URL=
```

Then run:

```
docker-compose up -d
```

## API documentation:

See [wallet.yaml](docs/wallet.yaml) - the content can be copied to [swagger editor](https://editor.swagger.io/)
