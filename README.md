# Ore CLI

Ore CLI is a command-line tool for interacting with the Ore program on Solana.

## Setup

**Prerequisites:**
- Install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

**Build:**
```sh
cargo build --release
```

**Configuration:**
- Write private keys to `keys`, one per line.
- Edit `RPC_URLS` in `supervision.sh` to include your desired RPC endpoints.

**Run:**
```sh
./supervision.sh
```

This will start Ore CLI using the specified configuration.

