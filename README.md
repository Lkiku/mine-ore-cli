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
- Write private keys to `keys.txt`, one per line.
- Write RPC endpoints to `rpcs.txt`, one per line.

**Run:**
```sh
./supervision.sh
```

If you have tmux, you can use this multiple run. If not, you can also install it. 
```sh
sudo apt-get install tmux
```

**multiple run:**
```sh
bash run.sh
```

This will start Ore CLI using the specified configuration.

