


<div align="center">

# 🦀 Rust Microservice Starter Kit
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
- [Built Using](#built_using)
- [Authors](#authors)

## 🧐 About <a name = "about"></a>
This Rust Microservice Starter Kit is designed to provide a solid foundation for building high-performance, type-safe, and memory-safe microservices. It leverages the power of Rust along with modern frameworks and tools to ensure robust and efficient API development.

**Visit Demo: https://rust-microservice-starter-kit.fly.dev **

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
├── migration
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── lib.rs
│       ├── m20240828_134140_create_feature_flag_table.rs
│       ├── m20240828_140244_seed_feature_flag_with_sample_data.rs
│       └── main.rs
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
    └── common
        ├── feature_flag_mock.rs
        └── mod.rs
```

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. 

### Prerequisites

1. Rust (latest stable version):
  Visit https://www.rust-lang.org/tools/install

2. SeaORM CLI:

  Open a terminal
  Run: `cargo install sea-orm-cli`


3. Neon PostgreSQL:
  Turbocharge your database with Neon! ⚡️ Serverless, autoscaling PostgreSQL in the cloud. Zero management, instant setup. Try Neon now and supercharge your app's performance!
  To get started with Neon:

  Visit https://neon.tech
  Sign up for an account
  Create a new project to get your database credentials

Alternatively, if you prefer local hosting:

PostgreSQL (local installation):

Download from https://www.postgresql.org/download/
### Installing

A step by step series of examples that tell you how to get a development env running.

1. Clone the repository
   ```
   git clone https://github.com/benborla/rust-microservice-starter-kit.git
   cd rust-microservice-starter-kit
   ```

2. Install dependencies
   ```
   cargo build; cd migration; cargo build
   ```

3. Set up the database
   ```
   # Create.env file via
   cp .env.dist .env
   
   # Run migrations
   cd migration
   cargo run -- up
   ```

4. Go back to the project, then run the application
   ```
   cargo run
   ```

The server should now be running on `http://localhost:3380`.

## 🔧 Running the tests <a name = "tests"></a>
You may find the tests files in `./tests`
Note: There's a sample Unit Test available, you may run it using
```
cargo test
```

## 🚀 Deployment <a name = "deployment"></a>
You may deploy this using [FlyIO](https://fly.io/).
## ⛏️ Built Using <a name = "built_using"></a>
- [Rust](https://www.rust-lang.org/) - Programming Language
- [Axum](https://github.com/tokio-rs/axum) - Web Framework
- [Tokio](https://tokio.rs/) - Asynchronous Runtime
- [SeaORM](https://www.sea-ql.org/SeaORM/) - ORM and Query Builder
- [SeaORM CLI](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/) - Official SeaORM CLI tool
- [Postgres](https://www.postgresql.org/) - Database
- [Neon](https://neon.tech/) - Serverless Postgres for modern developers

## ✍️ Authors <a name = "authors"></a>
- [@benborla](https://github.com/benborla) - Idea & Initial work
