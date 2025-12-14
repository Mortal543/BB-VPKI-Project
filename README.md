# BB-VPKI — Blockchain-Based Vehicular Public Key Infrastructure

BB-VPKI is a Rust-based research implementation of a vehicular Public Key Infrastructure (VPKI) that uses a lightweight in-memory blockchain to record certificate issuance and revocation events, combined with edge caching at roadside units (RSUs) and a pluggable ledger gateway interface (example: Hyperledger Fabric stub). The repository implements end-to-end flows for certificate issuance, revocation, authentication, message signing/verification (V2V), and basic blockchain storage management. This project is intended for academic/project evaluation of latency, throughput, and reliability properties of a BB-VPKI design.

## Key Features

- Certificate Authority (CA) layer with HSM abstraction
- Edge nodes (RSUs) with LRU cache for certificate validation
- In-memory blockchain with mining, pruning, and archival support
- Pluggable ledger gateway interface (Hyperledger Fabric stub provided)
- Vehicle On-Board Unit (OBU) SDK for signing and verification
- Comprehensive benchmarking harness that exports CSV reports
- Instrumentation for cache hit/miss, consensus latency, auth latencies, TPS, and storage footprint

## Architecture Overview

- `src/pki/` — Certificate Authority and certificate lifecycle management
- `src/edge/` — Edge node (RSU) implementation and caching logic
- `src/blockchain/` — Simple blockchain, transactions, block mining, and latency recording
- `src/network/` — Pluggable gateways (e.g., Hyperledger Fabric stub) and V2V network simulation
- `src/vehicle/` — On-board unit (OBU) SDK for signing and verifying messages
- `src/metrics/` — `PerformanceMetrics` model and CSV export utilities
- `src/main.rs` — Orchestration, system assembly, and benchmarking flow

The system is implemented with async Rust (Tokio) and uses `ed25519-dalek` for cryptographic operations.

## Build & Run

Prerequisites:

- Rust (recommended stable toolchain, 1.70+ tested; use `rustup`)
- `cargo` build tool

Build:

```bash
cd /path/to/bb-vpki
cargo build --release
```

Run the benchmark (release mode recommended):

```bash
cargo run --release
```

This will initialize the system, run the comprehensive benchmark, print a formatted report to stdout and save two CSV files in the repository root:

- `metrics.csv` — compact time-series of core metrics
- `metrics_detailed.csv` — human-readable detailed report

## Benchmarking Methodology

The `run_comprehensive_benchmark` routine in `src/main.rs` executes several stages and collects metrics:

1. Certificate issuance rate: concurrent issuance to CAs; dual-write to the in-memory blockchain and (optionally) to the ledger gateway.
2. Revocation latency: revoke a certificate via CA, broadcast revocation, and measure propagation + recording time.
3. Authentication delay: repeatedly authenticate a known-issued certificate via an RSU; the benchmark pre-warms the RSU cache with a real certificate and collects per-request latencies (microseconds).
4. Message signing & verification: measure average signing and verification time on an OBU.
5. Edge node cache hit/miss: atomic counters on the RSU report hit rate.
6. Blockchain throughput (TPS) and storage size: the local chain reports transactions per second and serialized storage footprint.
7. Consensus latency: recorded as the difference (ms) between block timestamp and each transaction timestamp at mining time; percentile summaries are provided.
8. System reliability: simple simulations of CA/RSU failures to verify basic fault-tolerant flow.
