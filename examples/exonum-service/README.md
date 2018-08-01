# Employees Database example

Minimal [Exonum](https://github.com/exonum/exonum) blockchain example implementing
a simple employees database.

## Prerequisites

To run this example you need to install [Rust](https://www.rust-lang.org/en-US/)
compiler and [third-party libraries](http://exonum.com/doc/get-started/install/).

## Build & Run

### Blockchain Node

To build and run a single node use:

```sh
# clone the repository with blockchain node
git clone git@github.com:eupn/exonum-employees.git
cd exonum/examples/employees

# Perform set of unit-tests of transaction logic and API using TestKit

cargo test

# build and create test nodes. 
cargo run -- generate-template common.toml --superuser-pkey 77471db2db2e2f0c1bde8cc222f790c707413447cebead035cde49a44a417188

cargo run -- generate-config common.toml  pub_1.toml sec_1.toml --peer-address 127.0.0.1:6331
cargo run -- generate-config common.toml  pub_2.toml sec_2.toml --peer-address 127.0.0.1:6332
cargo run -- generate-config common.toml  pub_3.toml sec_3.toml --peer-address 127.0.0.1:6333
cargo run -- generate-config common.toml  pub_4.toml sec_4.toml --peer-address 127.0.0.1:6334

cargo run -- finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 sec_1.toml node_1_cfg.toml --public-configs pub_1.toml pub_2.toml pub_3.toml pub_4.toml
cargo run -- finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 sec_2.toml node_2_cfg.toml --public-configs pub_1.toml pub_2.toml pub_3.toml pub_4.toml
cargo run -- finalize --public-api-address 0.0.0.0:8202 --private-api-address 0.0.0.0:8093 sec_3.toml node_3_cfg.toml --public-configs pub_1.toml pub_2.toml pub_3.toml pub_4.toml
cargo run -- finalize --public-api-address 0.0.0.0:8203 --private-api-address 0.0.0.0:8094 sec_4.toml node_4_cfg.toml --public-configs pub_1.toml pub_2.toml pub_3.toml pub_4.toml

cargo run -- run --node-config node_1_cfg.toml --db-path db1 --public-api-address 0.0.0.0:8200 &
cargo run -- run --node-config node_2_cfg.toml --db-path db2 --public-api-address 0.0.0.0:8201 &
cargo run -- run --node-config node_3_cfg.toml --db-path db3 --public-api-address 0.0.0.0:8202 &
cargo run -- run --node-config node_4_cfg.toml --db-path db4 --public-api-address 0.0.0.0:8203 &
```

Accounts can be created only by a **superuser**, so you have to provide a superuser *public* key in
configuration files. The easiest way to do so is to use `generate-template` subcommand
and provide a `--superuser-pkey` parameter as shown in example above.

Now the nodes are listening to HTTP requests on localhost and ports `:8200`, `:8201`, `:8202`, `:8203`

You can choose any of them to send an HTTP requests.

### REST API for the blockchain nodes

After a nodes started, you can choose any of them to send a `POST`/`GET` requests
in according to the REST API

#### REST API endpoints

List of endpoints supported by a node:

- `GET`, `/api/services/employees/v1/accounts`. Returns a list of all accounts in the storage
- `GET`, `/api/services/employees/v1/accounts/<pubkey>`. Returns a single account by its public key
- `POST`, `/api/services/employees/v1/accounts/transaction`. Accepts transactions to add, edit and delete accounts
- `GET`, `/api/services/employees/v1/blocks/<id>`. Returns a block height of an account with specific document ID


## License

This example is licensed under the Apache License (Version 2.0). See
[LICENSE](LICENSE) for details.