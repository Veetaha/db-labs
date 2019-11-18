# lab2

## Prerequisites

* [Rust toolchain](https://www.rust-lang.org/learn/get-started)
* libpq

## Bootstrap (Linux)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Add .cargo/bin to .bash_profile
echo 'export PATH="${HOME}/.cargo/bin:${PATH}"' >> ~/.bash_profile
# Update $PATH variable
source ~/.bash_profile
# Install libpq
sudo apt update
sudo apt install libpq-dev
# Install diesel_cli
cargo install diesel_cli --no-default-features --features postgres
```
