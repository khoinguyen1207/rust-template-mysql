<h1 align="center">Rust backend</h1>

<p align="center">
 <img src="https://img.shields.io/badge/Rust-EC4A3F?style=for-the-badge&logo=rust&logoColor=white"/>
  <img src="https://img.shields.io/badge/MySQL-4479A1?style=for-the-badge&logo=mysql&logoColor=white"/>
  <img src="https://img.shields.io/badge/Actix-000000?style=for-the-badge&logo=actix&logoColor=white"/>
</p>

## 💻 Technologies and Libraries Used

✅ **Framework**: Actix Web  
✅ **Language**: Rust  
✅ **Database**: MySQL  
✅ **Authentication**: JWT (token)  
✅ **Websocket**: Socket.io  
✅ **Test API**: Postman  
✅ **Deployment**: Docker & K8S  
✅ **API Documentation**: Swagger  
✅ **ORM (Object-Relational Mapper)**: Sqlx

## 📥 Installation

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

## Run application

```
cargo run
```

## Check clippy

```
cargo clippy -- -D warnings
```

## Format code

```
cargo fmt
```

## 📜 Contribution Guidelines

We welcome contributions to this project. If you have any ideas, suggestions, or bug reports, please create a new issue or submit a pull request.
