#  AEGIS - Distributed AI Inference Scheduler

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=for-the-badge&logo=rust)
![Distributed Systems](https://img.shields.io/badge/Distributed-AI%20Inference-blue?style=for-the-badge)
![Observability](https://img.shields.io/badge/Observability-Prometheus%20%7C%20OpenTelemetry-green?style=for-the-badge)
![Deployment](https://img.shields.io/badge/Deployment-Docker%20%7C%20Kubernetes-purple?style=for-the-badge)

### Advanced Engine for GPU Inference Scheduling

*A production-grade distributed AI inference orchestration platform built in Rust.*

[Architecture](#architecture) •
[Features](#features) •
[Quick Start](#quick-start) •
[Workspace](#workspace-layout) •
[Deployment](#deployment) •
[Documentation](#documentation)

</div>

---

## Overview

AEGIS is a distributed AI inference control plane designed to orchestrate large-scale model serving infrastructure.

Instead of relying on a single inference server, AEGIS coordinates requests across multiple nodes while providing:

- Intelligent request routing
- Distributed scheduling
- KV-cache-aware execution
- Speculative decoding support
- Runtime safety enforcement
- Consensus-backed state replication
- Cryptographic audit trails
- Production-grade observability

AEGIS acts as the coordination layer between clients and model-serving infrastructure, ensuring efficient, reliable, and scalable inference execution.

---

# Features

## 🚀 Distributed Scheduling

- Multi-node scheduling
- Failure detection
- Load balancing
- State replication
- Resource-aware placement

## 🧠 KV Cache Awareness

- Cache allocation management
- Cache reuse optimization
- Placement-aware scheduling
- Distributed cache coordination

## ⚡ Speculative Decoding

- Draft/verify execution flows
- Reduced inference latency
- Speculative execution primitives
- Benchmarking support

## 🛡 Runtime Safety

- Policy enforcement engine
- Runtime validation
- Monitoring hooks
- Fallback mechanisms

## 🔒 Verifiable Audit Trails

- Cryptographic event verification
- Immutable execution history
- Compliance-friendly logging
- Tamper detection

## 📊 Observability

- Prometheus metrics
- OpenTelemetry support
- Distributed tracing
- Grafana dashboards
- Health monitoring

## ☁ Cloud-Native Deployment

- Docker support
- Kubernetes manifests
- Horizontal scaling
- Production-ready infrastructure

---

# Architecture

```text
                    ┌──────────────────┐
                    │      Clients     │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │     Gateway      │
                    │ Auth • Queueing  │
                    │ Rate Limiting    │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │    Scheduler     │
                    │ Node Selection   │
                    │ Cache Placement  │
                    └────────┬─────────┘
                             │
           ┌─────────────────┼─────────────────┐
           ▼                 ▼                 ▼
 ┌────────────────┐ ┌────────────────┐ ┌────────────────┐
 │ Speculative    │ │ Safety Engine  │ │ Audit Engine   │
 │ Execution      │ │ Policies       │ │ Verification   │
 └────────────────┘ └────────────────┘ └────────────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │ Consensus Layer  │
                    │ State Replication│
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │ Telemetry Layer  │
                    │ Metrics & Traces │
                    └──────────────────┘
```

---

# Workspace Layout

```text
AEGIS/
├── gateway/                # Request ingress & API layer
├── scheduler/              # Distributed scheduling engine
├── speculative/            # Speculative decoding support
├── safety/                 # Runtime policy enforcement
├── audit/                  # Verifiable audit trail
├── telemetry/              # Metrics and tracing
├── consensus/              # Replicated state management
├── runtime/                # System orchestration
├── proto/                  # gRPC & protobuf definitions
├── benchmarks/             # Performance benchmarks
├── inference-backends/     # Backend integrations
├── docker/                 # Docker assets
├── kubernetes/             # Kubernetes manifests
├── README.md
├── ARCHITECTURE.md
├── DEPLOYMENT.md
└── PROJECT_OVERVIEW.md
```

---

# Workspace Crates

| Crate | Description |
|---------|------------|
| `gateway` | Request ingress, authentication, queueing, metrics |
| `scheduler` | Distributed scheduling and node coordination |
| `speculative` | Speculative decoding primitives |
| `safety` | Runtime safety policies and monitoring |
| `audit` | Cryptographically verifiable execution logging |
| `telemetry` | Shared observability infrastructure |
| `consensus` | Distributed state replication |
| `runtime` | End-to-end orchestration |
| `proto` | gRPC and protobuf definitions |
| `benchmarks` | System benchmarking suite |
| `inference-backends` | Backend abstraction layer |

---

# Quick Start

## Prerequisites

- Rust 1.75+
- Cargo
- Docker
- Docker Compose
- Kubernetes (optional)
- C/C++ Toolchain (for native backends)

---

## Clone Repository

```bash
git clone https://github.com/yadavkapil23/AEGIS.git
cd AEGIS
```

---

## Build Workspace

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

---

## Run Local Cluster

```bash
docker-compose up -d
```

Services:

| Service | URL |
|----------|-----|
| Node 1 | http://localhost:8000 |
| Node 2 | http://localhost:8001 |
| Node 3 | http://localhost:8002 |
| Prometheus | http://localhost:9090 |
| Grafana | http://localhost:3000 |

Health check:

```bash
curl http://localhost:8000/health
```

Metrics:

```bash
curl http://localhost:8000/metrics
```

Shutdown:

```bash
docker-compose down
```

---

## Run Scheduler Directly

```bash
cargo run \
  --release \
  -p aegis-scheduler \
  --bin aegis-scheduler-node
```

---

# Testing

## Full Workspace

```bash
cargo test --release
```

## Integration Tests

```bash
cargo test -p aegis-scheduler --test chaos_tests -- --nocapture

cargo test -p aegis-scheduler --test failure_recovery_tests -- --nocapture

cargo test -p aegis-scheduler --test network_hardening_tests -- --nocapture

cargo test -p aegis-scheduler --test integration_3node -- --nocapture
```

---

# Benchmarks

```bash
cargo bench -p aegis-benchmarks

cargo bench -p aegis-scheduler

cargo bench -p aegis-inference-backends
```

---

# Deployment

## Docker

```bash
docker build -t aegis .
```

Run:

```bash
docker-compose up -d
```

## Kubernetes

```bash
kubectl apply -f kubernetes/
```

Includes:

- Namespaces
- RBAC
- StatefulSets
- Services
- HPA
- Network Policies

---

# Observability

AEGIS ships with built-in observability support:

### Metrics

- Prometheus

### Tracing

- OpenTelemetry
- tracing-rs

### Dashboards

- Grafana

Useful files:

```text
prometheus.yml
grafana-datasources.yml
METRICS.md
OPERATIONAL_RUNBOOKS.md
```

---

# Documentation

## Getting Started

- `GETTING_STARTED.md`
- `PROJECT_OVERVIEW.md`

## Architecture

- `ARCHITECTURE.md`

## Deployment

- `DEPLOYMENT.md`
- `DEPLOYMENT_CHECKLIST.md`

## Operations

- `OPERATIONAL_RUNBOOKS.md`

---

# Current Status

### Implemented

✅ Multi-crate Rust workspace

✅ Distributed scheduler

✅ gRPC communication layer

✅ Consensus primitives

✅ KV-cache coordination

✅ Safety monitoring

✅ Audit infrastructure

✅ Observability stack

✅ Docker deployment

✅ Kubernetes manifests

### Roadmap

- Advanced scheduling policies
- Dynamic GPU resource allocation
- Multi-region coordination
- Additional backend integrations
- Enhanced speculative execution strategies

---

# Contributing

```bash
git checkout -b feature/my-change

cargo fmt

cargo clippy

cargo test --release
```

Before making major changes, identify the owning crate and review its associated tests and documentation.

---

# License

See the repository's LICENSE file for licensing information.

---

<div align="center">

Built with ❤️ in Rust for large-scale AI inference systems.

</div>
