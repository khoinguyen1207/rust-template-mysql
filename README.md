# Rust backend template with mysql and sqlx

## Migrate DB

Install Sqlx

```
cargo install sqlx-cli --no-default-features --features postgres
```

Add migrate

```
sqlx migrate add {sql-script-name}
```

Run Migrate

```
sqlx migrate run
```

## Check clippy

```
cargo clippy -- -D warnings
```

## Format code

```
cargo fmt
```
