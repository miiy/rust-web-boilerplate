# rust-web-boilerplate

Rust web boilerplate with Actix Web, SQLx, redis-rs, Askama

## Features

- CORS

- MySQL

- Redis

- Askama

Actix Web: <https://github.com/actix/actix-web>

SQLx: <https://github.com/launchbadge/sqlx>

redis-rs: <https://github.com/redis-rs/redis-rs>

Askama: <https://github.com/djc/askama>

## Getting Started

create database

```sql
CREATE DATABASE rust_test DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

migrate

```bash
cargo install sqlx-cli
export DATABASE_URL=mysql://root:123456@localhost/rust_test
sqlx migrate run
```

set environment variables:

```bash
cp .env.example .env
```

## Running

```bash
cargo run
```
