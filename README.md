# BB-VPKI — Blockchain-Based Vehicular Public Key Infrastructure

Author: Edesiri (Project)
Date: December 2025

---

## Abstract

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

The benchmark exports both averages and percentiles (p50/p95/p99) for authentication and consensus latencies for tail-latency analysis.

## Results & Metrics (Representative Run)

Below are representative results from a benchmark run performed during development. They are included here to make it easy for a project reader to see the system behavior and the CSV mapping.

- Timestamp: 2025-12-04 (representative)
- Certificate issuance rate: ~19.26 certs/sec (with Hyperledger Fabric gateway enabled — dual-write mode)
- Revocation latency: ~0 ms (revocation flow completed locally; zero indicates a fast local operation or not-measured path)
- Authentication delay (average): ~0.05 — 0.14 μs (average of many requests)
- Authentication percentiles: p50/p95/p99 — typically very low (microsecond-level) in these controlled runs
- Message signing time: ~40 — 48 μs
- Message verification time: ~57 — 69 μs
- Edge cache hit rate: ~19.96% (with the current benchmark workload and LRU size)
- Blockchain throughput: ~80 TPS (measured as transactions per second over a measurement window)
- Consensus latency (avg): ~50 ms (when using the Fabric gateway stub which simulates submit latency)
- Consensus percentiles (p50/p95/p99): reported from mined transaction timestamps — typically near the gateway's simulated latency
- Blockchain serialized size: ~0.41 MB (small in-memory chain for these runs)
- Pruned blocks: varies (benchmark prunes older blocks; see CSV for the exact value)

Notes about the numbers:

- Some early runs showed extremely high consensus latency due to mis-aggregation; the current code stores per-transaction consensus latencies and reports percentiles.
- Authentication latencies are measured with nanosecond precision and stored/reported in microseconds; very small values may appear as `0` μs when they round down.
- Cache hit rate is workload-dependent: the benchmark pre-warms the RSU cache with a valid certificate, but most authentication requests in the synthetic load are intentionally cache-miss heavy to exercise the chain lookup path.

## CSV Schema (compact `metrics.csv`)

Columns written (compact CSV):

- `timestamp` — ISO8601 timestamp of the benchmark run
- `certificate_issuance_rate_certs_per_sec` — average issuance throughput
- `revocation_latency_ms` — revocation latency in milliseconds
- `authentication_delay_us` — average authentication delay in microseconds
- `authentication_p50_us`, `authentication_p95_us`, `authentication_p99_us` — authentication latency percentiles (μs)
- `message_signing_time_us` — average message signing time (μs)
- `message_verification_time_us` — average message verification time (μs)
- `cache_hit_rate_percent` — RSU cache hit rate (%)
- `cache_miss_rate_percent` — RSU cache miss rate (%)
- `consensus_latency_ms` — average consensus latency (ms)
- `consensus_p50_ms`, `consensus_p95_ms`, `consensus_p99_ms` — consensus percentiles (ms)
- `blockchain_tps` — approximate transactions per second measured
- `blockchain_size_mb` — serialized chain size in megabytes
- `pruned_blocks` — number of blocks pruned during the run
- `deprecated_certificates` — number of archived deprecated certificates
- `system_uptime_secs` — system uptime when the benchmark finished

## Interpreting the Results (Project

Guidance)

- Authentication latency (avg vs percentiles): For safety-critical vehicular systems, tail latencies (95th/99th percentile) are more important than the average. The implementation provides those percentiles to support Project discussion on real-time behavior and outliers.
- Consensus latency vs TPS: In many ledger integrations, submit latency (time-to-finality) and throughput trade off. The provided Fabric gateway stub simulates submit latency (~50 ms) to let you reason about expected consensus behavior; replace the stub with a real client to measure production values.
- Cache effectiveness: RSU cache size and workload shape cache hit rates. For higher on-path authentication success, tune cache size and pre-warm strategy.

## Limitations & Future Work

- The Hyperledger Fabric integration is a stub; for publication-grade results, integrate a real Fabric client and run with a real ordering service.
- The blockchain is in-memory and simplistic (teaching/research purpose) — migrate to a persistent ledger for durability experiments.
- The benchmark workload is synthetic and should be complemented with trace-driven workloads for real-world evaluation.
- Percentile calculation uses a simple nearest-rank approach; for small samples consider interpolation.

## How to Cite / Project Notes

If you use this code or the benchmark results in academic work, cite the repository and include a note that the implementation is a research prototype. Include measurement methodology (benchmark seed, host machine, workload parameters) in your project appendix for reproducibility.

## Contributing

Contributions are welcome. Typical contributions for a project extension:

- Replace the Fabric stub with a real SDK client and measure submit/finality times
- Add trace-driven benchmark workloads or replay recorded vehicular traces
- Add persistent storage to the blockchain and re-run storage/IO experiments
- Improve percentile/interpolation statistics and reporting

## Contact

For questions about the implementation or the project experiment design, open an issue in this repository or contact the author via the repository account.

---

End of README
