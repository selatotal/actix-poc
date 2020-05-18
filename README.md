# actix-poc
Microservice using Actix/Diesel

# Configure Diesel

* Configure Diesel client and create initial database
```bash
sudo apt install libpq-dev libmysqlclient-dev libsqlite3-dev
cargo install diesel_cli
echo DATABASE_URL="mysql://user:password@example.com/actix-poc" > .env
diesel setup
```
* Running migration
```
diesel migration run
```

# How to Run
```bash
cargo run
```

# Endpoints
Import the [Postman Collection](postman/ActixPoc.postman_collection.json)

# References

## Documentation
* [Actix](https://actix.rs): Rust's powerful actor system and most fun web framework
* [Diesel](https://diesel.rs): Diesel is a Safe, Extensible ORM and Query Builder for Rust

## Crates
* [actix-rt](https://crates.io/crates/actix-rt): Actix runtime
* [actix-web](https://crates.io/crates/actix-web): Actix web framework is simple, pragmatic, extremely fast, and for Rust.
* [diesel](https://crates.io/crates/diesel): A safe, extensible ORM and Query Builder for Rust
* [dotenv](https://crates.io/crates/dotenv): Storing configuration in the environment using .env files
* [pretty_env_logger](https://crates.io/crates/pretty_env_logger) A simple logger built on top off env_logger. It is configured via an environment variable and writes to standard error with nice colored output for log levels.
* [log](https://crates.io/crates/log): A Rust library providing a lightweight logging facade.
* [serde](https://crates.io/crates/serde): Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
* [serde_json](https://crates.io/crates/serde_json): Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
* [uuid](https://crates.io/crates/uuid): Generate and parse UUIDs.
