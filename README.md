# Blockchain Project in Rust

[![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](#license)

A compact, educational proof‑of‑history (PoH) blockchain prototype written entirely in Rust.  
The goal is to demonstrate core ledger concepts—transaction validation, PoH sequencing,
and block construction—without external dependencies or networking layers.

---

## ✨ Features

- **Proof‑of‑History (PoH)** — deterministic tick sequence linking every block  
- **Cryptography**: BLAKE3 hashing for both PoH ticks and block IDs  
- **Serde‑powered** JSON (de)serialization for transactions & committed blocks  
- **Single‑binary demo**: `cargo run` spins up a mini‑chain and prints the results

---

## 🛠️ Prerequisites

| Requirement | Minimum version | Install |
|-------------|-----------------|---------|
| Rust tool‑chain | **1.78** or newer | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Build tools  | gcc / make      | Debian: `sudo apt install build-essential pkg-config` |

---

## 🚀 Quick start

```bash
# 1. Clone the repo
git clone https://github.com/jamcam245/blockchain-project-in-rust.git
cd blockchain-project-in-rust

# 2. Build & run the demo
cargo run
