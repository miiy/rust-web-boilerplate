# sqlx-cli

install

```bash
cargo install sqlx-cli
```

create migrate

```bash
sqlx migrate add -r users
```

migrate

```bash
export DATABASE_URL=mysql://root:123456@localhost/rust_web
sqlx migrate run
```

revert

```bash
sqlx migrate revert
```
