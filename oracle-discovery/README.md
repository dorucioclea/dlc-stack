# oracle-discovery

Oracle discovery is responsible register / verify (explicitly or implecitly) oracle addresses and public keys.

## Endpoionts

- `/oracles`: Listing verified oracles
- `/unverified_oracles`: Listing unverified oracles
- `/register`: Register a new oracle (address)
- `/verify`: Verify an oracle (also gathers oracle public key), if successful, the oracle will be moved from unverified to verified ones.

## Configurations

Following environment variables can be set:

- `ORACLE_URLS`: a list of comma separated addresses of oracles, these oracles will go automatically to be verified during startup
- `URL_REPLACE_RULES`: comma separated URLs, that will replace URL matches with new values (pattern and replace are separated by `=`). It can be useful for networks where the internal/external addresses are different. E.g.: oracle:8080=localhost:8080,oracle2:8080=localhost:8081,oracle3:8080=localhost:8082
