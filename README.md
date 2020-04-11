# actix-poc
Microservice using Actix/Diesel

# Configure Diesel

Configure Diesel client and create initial database
```bash
sudo apt install libpq-dev libmysqlclient-dev
cargo install diesel_cli
echo DATABASE_URL=file:customer.db > .env
diesel setup
```

Creating customer table
```
diesel migration generate create_customer
```

# How to Run
```bash
cargo run
```
If you want debug (or other level) in logs
```bash
LOG_LEVEL=debug cargo run
```

# References

## Documentation
* [Actix](https://actix.rs): Rust's powerful actor system and most fun web framework

## Crates
* [actix-rt](https://crates.io/crates/actix-rt): Actix runtime
* [actix-web](https://crates.io/crates/actix-web): Actix web framework is simple, pragmatic, extremely fast, and for Rust.
* [pretty_env_logger](https://crates.io/crates/pretty_env_logger) A simple logger built on top off env_logger. It is configured via an environment variable and writes to standard error with nice colored output for log levels.
* [log](https://crates.io/crates/log): A Rust library providing a lightweight logging facade.
* [serde](https://crates.io/crates/serde): Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
* [serde_json](https://crates.io/crates/serde_json): Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
