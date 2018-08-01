#!/bin/sh

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
