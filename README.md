
# Rust Microservice Starter Kit

<div align="center">

  [![Status](https://img.shields.io/badge/status-active-success.svg)]() 
  [![GitHub Issues](https://img.shields.io/github/issues/benborla/rust-microservice-starter-kit.svg)](https://github.com/benborla/rust-microservice-starter-kit/issues)
  [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/benborla/rust-microservice-starter-kit.svg)](https://github.com/benborla/rust-microservice-starter-kit/pulls)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> A high-performance, type-safe, and memory-safe Microservice built with Rust. This starter kit provides a robust API, leveraging the power and safety of Rust along with the Axum web framework and Neon for PostgreSQL database interactions.
    <br> 
</p>

## 📝 Table of Contents
- [About](#about)
- [Getting Started](#getting_started)
- [Deployment](#deployment)
- [Usage](#usage)
- [Built Using](#built_using)
- [TODO](../TODO.md)
- [Contributing](../CONTRIBUTING.md)
- [Authors](#authors)
- [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>
This Rust Microservice Starter Kit is designed to provide a solid foundation for building high-performance, type-safe, and memory-safe microservices. It leverages the power of Rust along with modern frameworks and tools to ensure robust and efficient API development.

### Key Features:
- **Rust**: A language empowering everyone to build reliable and efficient software.
- **Axum**: A modular web framework that's built with Tokio, Tower, and Hyper.
- **Neon**: Serverless Postgres with a generous free tier. Neon separates storage and compute to offer autoscaling, branching, and bottomless storage.
- **High Performance**: Optimized for speed and efficiency.
- **Type Safety**: Leveraging Rust's strong type system to prevent runtime errors.
- **Memory Safety**: Rust's ownership model ensures memory safety without garbage collection.

## 🏁 Getting Started <a name = "getting_started"></a>

### Project Structure
```
./rust-microservice-starter-kit
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── README.md
├── docs
│   └── API.md
├── fly.toml
├── src
│   ├── api
│   │   ├── handlers
│   │   │   ├── feature_flags.rs
│   │   │   ├── health_check.rs
│   │   │   └── mod.rs
│   │   ├── mod.rs
│   │   └── routes.rs
│   ├── config.rs
│   ├── db
│   │   └── mod.rs
│   ├── error.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── models
│   │   ├── feature_flags.rs
│   │   ├── mod.rs
│   │   └── prelude.rs
│   └── services
│       ├── feature_flag_service.rs
│       └── mod.rs
└── tests
    ├── api_tests.rs
    └── service.tests.rs

```

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See [deployment](#deployment) for notes on how to deploy the project on a live system.

### Prerequisites

What things you need to install the software and how to install them.

```
- Rust (latest stable version)
- PostgreSQL
- Docker (optional, for containerization)
- WIP
```

### Installing

A step by step series of examples that tell you how to get a development env running.

1. Clone the repository
   ```
   git clone https://github.com/benborla/rust-microservice-starter-kit.git
   cd rust-microservice-starter-kit
   ```

2. Install dependencies
   ```
   cargo build
   ```

3. Set up the database
   ```
   # Create.env file via
   cp .env.dist .env
   
   # Run migrations
   cargo run --bin migrate
   ```

4. Run the application
   ```
   cargo run
   ```

The server should now be running on `http://localhost:8080`.

## 🔧 Running the tests <a name = "tests"></a>
Explain how to run the automated tests for this system.

```
cargo test
```

## 🎈 Usage <a name="usage"></a>
-- TODO --

## 🚀 Deployment <a name = "deployment"></a>
-- TODO --

## ⛏️ Built Using <a name = "built_using"></a>
- [Rust](https://www.rust-lang.org/) - Programming Language
- [Axum](https://github.com/tokio-rs/axum) - Web Framework
- [Tokio](https://tokio.rs/) - Asynchronous Runtime
- SQLX - async, pure Rust SQL crate featuring compile-time checked queries without a DSL.
- [SeaORM](https://www.sea-ql.org/SeaORM/) - ORM and Query Builder
- [SeaORM CLI](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/) - Official SeaORM CLI tool
- [Postgres](https://www.postgresql.org/) - Database
- [Neon](https://neon.tech/) - Serverless Postgres for modern developers
- [Docker](https://www.docker.com/) - Containerization

## ✍️ Authors <a name = "authors"></a>
- [@benborla](https://github.com/benborla) - Idea & Initial work

See also the list of [contributors](https://github.com/benborla/rust-microservice-starter-kit/contributors) who participated in this project.

## 🎉 Acknowledgements <a name = "acknowledgement"></a>
- WIP
