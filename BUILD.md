# Building the Lexe sidecar from source

Lexe provides precompiled binaries for the Lexe sidecar, but it can also be
built from source if your platform is not supported or you prefer the extra
security.

## Steps

Install `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#  default host triple: default
#    default toolchain: stable
#              profile: default
# modify PATH variable: yes
```

Clone the `lexe-public` monorepo

```bash
git clone https://github.com/lexe-app/lexe-public
cd lexe-public
```

Build the sidecar and copy it to somewhere in PATH, for example `/usr/local/bin`:

```bash
cargo build --release -p sdk-sidecar --bin lexe-sidecar
sudo cp target/release/lexe-sidecar /usr/local/bin
```

Run the sidecar:

```bash
lexe-sidecar --help
```

Update the sidecar frequently to ensure your app continues to run smoothly:

```bash
cd lexe-public
git pull
cargo build --release -p sdk-sidecar --bin lexe-sidecar
sudo cp target/release/lexe-sidecar /usr/local/bin
```

See the [Quickstart](quickstart.md) page for remaining setup.
