# rust-web-boilerplate

Rust web boilerplate with Actix Web, Tera, Vue 3, Vite

# Required

## Features

- Modular design
- Http server: [Actix Web](https://github.com/actix/actix-web)
- Time: [time-rs](https://github.com/time-rs/time)
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
├── frontend      // frontend files
├── migrations    // database migrations files
├── src
│ ├── server      // web modules
│ │ ├── auth      // auth module
│ │ ├── index     // index module
│ │ ├── error.rs  // web error
│ │ ├── mod.rs
│ │ └── route.rs  // web routes
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

WEB module structure

```text
├── src
│ ├── server
│ │ ├── post
│ │ │ ├── handler.rs  // handler
│ │ │ ├── service.rs  // service
│ │ │ ├── mod.rs
│ │ │ ├── route.rs    // module routes
│ │ │ ├── dto.rs      // data transfer object
│ │ │ ├── template.rs // tera templates files
```

## Modules

WEB

```text
Index

Health

Auth

Post
```

## Getting Started
