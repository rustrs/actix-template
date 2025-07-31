# Rust Actix-Web Starter Template

A production-ready **Rust web backend template** built with [`actix-web 4`](https://actix.rs/), designed for rapid API development with JWT authentication, Redis session management, modular routing, and best practices for scalability and maintainability.

## ✨ Features

- ⚙️ Actix-Web 4 with modular structure  
- 🔐 JWT-based authentication middleware  
- 🧠 Redis integration using [`deadpool-redis`](https://crates.io/crates/deadpool-redis)  
- 📁 Clean project layout (routes, services, middleware, config)  
- ♻️ Graceful error handling and typed responses  
- 🧪 Ready for integration with testing & logging  

📁 Project Structure

```
.
├── app
│   ├── mod.rs
│   └── state.rs
├── config
│   ├── config.rs
│   └── mod.rs
├── constants.rs
├── db
│   ├── mod.rs
│   ├── mysql.rs
│   └── redis.rs
├── handlers
│   ├── health.rs
│   ├── mod.rs
│   └── user.rs
├── lib.rs
├── main.rs
├── middleware
│   ├── auth.rs
│   ├── mod.rs
│   └── rate_limit.rs
├── models
│   ├── mod.rs
│   └── user.rs
├── mq
├── routes
│   ├── health.rs
│   ├── mod.rs
│   └── user.rs
├── services
└── utils
    ├── error_code.rs
    ├── logger.rs
    ├── mod.rs
    ├── response.rs
    └── utils.rs
```

## 🚀 Getting Started

```bash
git clone https://github.com/rustrs/actix-template.git
cd template
cargo run -- -c config.toml
```

OR

```
cargo install cargo-generate
cargo generate --git https://github.com/rustrs/actix-template.git --name my-new-project

```

```
➜  airdrop git:(master) ✗ cargo run -- --help
   Compiling airdrop v0.1.0 (/Users/xiaohai/work/code/me/rust/airdrop)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.66s
     Running `target/debug/airdrop --help`
Usage: airdrop [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to the configuration file [default: config/local.toml]
  -h, --help             Print help
  -V, --version          Print version
➜  airdrop git:(master) ✗ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.65s
➜  airdrop git:(master) ✗ cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/airdrop --help`
Usage: airdrop [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to the configuration file [default: config/local.toml]
  -h, --help             Print help
  -V, --version          Print version

```


🔒 Authentication Flow
- JWT issued on login and stored client-side
- On each request, token is verified and checked against Redis (supports blacklist)
- Logout adds token to Redis blacklist with expiration


📦 Dependencies
- actix-web
- actix-service
- jsonwebtoken
- serde
- serde_json
- deadpool-redis
- redis