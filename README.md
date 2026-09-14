# Matchbook

A matching engine that keeps an order book. This project was given to me from Claude Opus 4.8, with the following prompt

```md
Give me a project to work on, tell me the requirements, tech stack and features to put into it. MVP it, give it a name.
```
## Tech stack
Engine Core: Rust
Frontent: React and Typescript
Transport: WebSocket (Rust axum and tokio-tungstenite)
Load generation and Testing: Python
Deployment: Docker

## MVP scope
- Engine accepts limit orders, market orders, and cancels.
- Matches by price-time priority with partial fills, emits a trade per match.
- WebSocket feed pushes a book snapshot on connect, then deltas.
- React ladder shows top 10 levels each side + a trades tape, updating live.
- Benchmark harness prints orders/sec and p50/p99 match latency.

## Requirements
Functional:

- Accept limit and market orders (buy/sell), plus cancels.
- Match by strict price-time priority (best price first, FIFO within a price level).
- Handle partial fills and emit a trade record per match.
- Broadcast book snapshots + incremental deltas over WebSocket.
- Deterministic replay: same input sequence in, identical trades out.

Non-functional (this is where the depth lives):

- In-memory, single-threaded core so it's deterministic and easy to reason about.
- A benchmark harness that reports orders/sec and latency percentiles.
- Correctness tested against a naive reference matcher on randomized order streams.

