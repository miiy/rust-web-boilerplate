# rust-web-boilerplate

Rust web boilerplate with Actix Web, SQLx, redis-rs, Tera

# Required

- MySQL
- Redis

## Features

- Modular design
- Http server: [Actix Web](https://github.com/actix/actix-web)
- Time: [time-rs](https://github.com/time-rs/time)
- Database: MySQL [SQLx](https://github.com/launchbadge/sqlx) 
- Cache: Redis [redis-rs](https://github.com/redis-rs/redis-rs)
- Json: [serde](https://github.com/serde-rs/serde)
- Config: [config-rs](https://github.com/rust-cli/config-rs)
- CORS
- Static files
- Template engine: [Tera](https://github.com/Keats/tera)
- Session: [actix-session](https://github.com/actix/actix-extras/tree/master/actix-session)

## Project structure

The project adopts a modular architectural design.

```text
├── config        // app config files
├── docs          // documents
├── migrations    // database migrations files
├── src
│ ├── api         // api modules
│ │ ├── user      // user module
│ │ ├── post      // post module
│ │ ├── error.rs  // api error
│ │ ├── mod.rs
│ │ └── routes.rs // api routes
│ ├── middleware  // actix middleware
│ ├── web         // web modules
│ │ ├── auth      // auth module
│ │ ├── index     // index module
│ │ ├── error.rs  // web error
│ │ ├── mod.rs
│ │ └── routes.rs // web routes
│ ├── lib.rs
│ ├── main.rs
│ ├── config.rs   // app config
│ └── db.rs
├── static
├── templates
├── Cargo.toml
├── Cargo.lock
├── LICENSE
└── README.md
```

API module structure

```text
├── src
│ ├── api
│ │ ├── post            // module name
│ │ │ ├── error.rs      // error
│ │ │ ├── handler.rs    // handler
│ │ │ ├── service.rs    // service logic
│ │ │ ├── mod.rs
│ │ │ ├── routes.rs     // module routes
│ │ │ ├── repository.rs // repository
│ │ │ ├── dto.rs        // request and response struct
│ │ │ └── model.rs      // model
```

WEB module structure

```text
├── src
│ ├── web
│ │ ├── post
│ │ │ ├── handler.rs  // handler
│ │ │ ├── service.rs  // service
│ │ │ ├── mod.rs
│ │ │ ├── routes.rs   // module routes
│ │ │ ├── dto.rs      // data transfer object
│ │ │ ├── template.rs // tera templates files
```

## Modules

API 

```text
Auth: register, login

User: me

Post: create, update, delete, detail, list
```

WEB

```text
Index

Health

Auth

Post
```

## Getting Started

create database

```sql
CREATE DATABASE rust_web DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

migrate

```bash
cargo install sqlx-cli
export DATABASE_URL=mysql://root:123456@localhost/rust_web
sqlx migrate run
```

edit config

config/*.yaml

## Running

```bash
cargo run
```
