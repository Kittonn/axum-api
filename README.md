# Axum API & User Consumer Workspace

## ℹ️ Overview

A Proof of Concept (PoC) to evaluate **Rust** for production-ready microservices. The goal is to verify if the Rust ecosystem (Axum, Tokio, SQLx) is mature enough for real-world application development, focusing on performance, type safety, and developer experience.

## 🛠 Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: MSSQL (via `sqlx`, `tiberius`)
- **Messaging**: Kafka (via `rdkafka`)
- **gRPC**: Tonic
- **Cache**: Redis
- **Infra**: Docker Compose

## 📂 Project Structure

```
.
├── crates
│   ├── axum-api       # REST API Service
│   └── user-consumer  # Kafka Consumer Service
├── k6                 # Load Testing Scripts
├── proto              # gRPC Protobufs
├── scripts            # Utility Scripts
└── docker-compose.yml # Infrastructure Orchestration
```

## 🚀 How to Run

### 1. Start All Servers

Start all services including the REST API, Kafka Consumer, and Infrastructure:

- **REST API**: `:4000`
- **Kafka UI**: `:8080`

```bash
cp crates/axum-api/.env.example crates/axum-api/.env
cp crates/user-consumer/.env.example crates/user-consumer/.env
```

```bash
docker compose up -d --build
```

### 2. Run k6 Benchmark

The benchmark suite tests the "User Registration" flow.

```bash
k6 run k6/load-test.js --summary-export=summary.json
```
