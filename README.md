<div align="center">

# 🦀 Rust Microservice Starter Kit
  [![Status](https://img.shields.io/badge/status-active-success.svg)]() 
  [![GitHub Issues](https://img.shields.io/github/issues/benborla/rust-microservice-starter-kit.svg)](https://github.com/benborla/rust-microservice-starter-kit/issues)
  [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/benborla/rust-microservice-starter-kit.svg)](https://github.com/benborla/rust-microservice-starter-kit/pulls)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> 
A high-performance, type-safe, and memory-safe microservice starter kit built with Rust. This starter kit provides a robust foundation for building efficient APIs, leveraging the power and safety of Rust along with the Axum web framework and Neon for serverless PostgreSQL database interactions.
</p>

## 📝 Table of Contents
- [About](#about)
  - [Key Features](#key-features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Project Structure](#project-structure)
- [Running Tests](#running-tests)
- [Deployment](#deployment)
- [Built With](#built-with)
- [Authors](#authors)

## 🧐 About <a name="about"></a>
This Rust Microservice Starter Kit provides a solid foundation for building high-performance, type-safe, and memory-safe microservices. It harnesses the power of Rust along with modern frameworks and tools to ensure robust and efficient API development.

**Visit Demo:** [[https://rust-microservice-startup-kit.benborla.dev/](https://rust-microservice-starter-kit.fly.dev)](https://rust-microservice-starter-kit.fly.dev/) [Updated]

###

### Key Features <a name="key-features"></a>
- **Rust**: A language empowering everyone to build reliable and efficient software.
- **Axum**: A modular web framework built with Tokio, Tower, and Hyper.
- **Neon**: 🚀 Supercharge your development with Neon's serverless PostgreSQL! Enjoy lightning-fast performance, effortless scaling, and a generous free tier. Say goodbye to database management hassles and hello to the future of PostgreSQL! 🌟
- **High Performance**: Optimized for speed and efficiency.
- **Type Safety**: Leveraging Rust's strong type system to prevent runtime errors.
- **Memory Safety**: Rust's ownership model ensures memory safety without garbage collection.

## 🏁 Getting Started <a name="getting-started"></a>

### Prerequisites <a name="prerequisites"></a>

1. **Rust (latest stable version)**
   - Visit https://www.rust-lang.org/tools/install
   - Follow the instructions for your operating system

2. **SeaORM CLI**
   Open a terminal and run:
   ```
   cargo install sea-orm-cli
   ```

3. **Neon PostgreSQL**
   🎉 Elevate your database game with Neon! 🚀 Experience the power of serverless, autoscaling PostgreSQL in the cloud. Zero management, instant setup, and a generous free tier to get you started. Unleash the full potential of your applications with Neon's unrivaled performance and flexibility. Try Neon now and witness the revolution in PostgreSQL hosting! ⚡️💪
   
   To get started with Neon:
   - Visit https://neon.tech
   - Sign up for an account
   - Create a new project to get your database credentials

   Alternatively, if you prefer local hosting:
   
   **PostgreSQL (local installation)**
   - Download from https://www.postgresql.org/download/
   - Follow the installation guide for your operating system

### Installation <a name="installation"></a>

1. Clone the repository
   ```
   git clone https://github.com/benborla/rust-microservice-starter-kit.git
   cd rust-microservice-starter-kit
   ```

2. Install dependencies
   ```
   cargo build
   cd migration && cargo build
   ```

3. Set up the database
   ```
   # Create .env file 
   cp .env.dist .env
   # Edit your .env and replace the value of DATABASE_URL,
   # it should be a POSTGRESQL Connection String from your Neon database or your local PostgreSQL server.

   # Run migrations
   cd migration
   cargo run -- up  
   ```

4. Run the application
   ```
   cd ..
   cargo run
   ```

The server should now be running on `http://localhost:3000`.

## 🏗️ Project Structure <a name="project-structure"></a>

```
./rust-microservice-starter-kit
├── src
│   ├── api
│   │   ├── handlers
│   │   ├── mod.rs
│   │   └── routes.rs
│   ├── config.rs  
│   ├── db
│   │   └── mod.rs
│   ├── error.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── models  
│   │   ├── mod.rs
│   │   └── prelude.rs
│   └── services
│       └── mod.rs
├── migration
│   └── src 
│       └── ...
├── tests
│   ├── api_tests.rs
│   └── common
│       └── mod.rs
├── ...
```

## 🔧 Running Tests <a name="running-tests"></a>

You can find the test files in the `./tests` directory. 
Note: A sample unit test is available. To run the tests, use:

```
cargo test
```

## 🚀 Deployment <a name="deployment"></a>

You can deploy this project using [Fly.io](https://fly.io/).

## 🛠️ Built With <a name="built-with"></a>

- [Rust](https://www.rust-lang.org/) - Programming Language
- [Axum](https://github.com/tokio-rs/axum) - Web Framework 
- [Tokio](https://tokio.rs/) - Asynchronous Runtime
- [SeaORM](https://www.sea-ql.org/SeaORM/) - ORM and Query Builder
- [SeaORM CLI](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/) - Official SeaORM CLI tool
- [PostgreSQL](https://www.postgresql.org/) - Database
- [Neon](https://neon.tech/) - 🌟 Serverless PostgreSQL for the modern developer 🚀

## ✍️ Authors <a name="authors"></a>

- [@benborla](https://github.com/benborla) - Idea & Initial work
