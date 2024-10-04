#!/bin/bash

sudo apt install -y pkg-config libssl-dev
sudo apt install -y build-essential
export OPENSSL_DIR=/usr/bin/openssl

cargo install sqlx-cli
cargo install sqlx-cli --verbose

cargo add argon2
cargo add async-trait
cargo add chrono --features serde
cargo add dotenv
cargo add jsonwebtoken
cargo add serde --features derive
cargo add serde_json
cargo add sqlx --features runtime-async-std-native-tls,postgres,chrono,uuid,runtime-tokio-native-tls
cargo add uuid --features serde,v4
cargo add validator --features derive
cargo add axum --features multipart
cargo add axum-extra --features cookie
cargo add tokio --features full
cargo add tokio-cron-scheduler
cargo add tower
cargo add time
cargo add tower-http --features cors,trace
cargo add tracing-subscriber
cargo add aes
cargo add block-modes
cargo add rsa
cargo add rand
cargo add base64
