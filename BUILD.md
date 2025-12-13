# Building the `lexe-sidecar` binary

Lexe provides precompiled binaries for the `lexe-sidecar`, but the sidecar can
also be built from source if your platform is not provided or you prefer the
extra security.

## Building from source

Install `rustup`

```bash
$ curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs | bash

#  default host triple: default
#    default toolchain: stable
#              profile: default
# modify PATH variable: yes
```

Clone the `lexe-public` monorepo

```bash
$ git clone https://github.com/lexe-app/lexe-public
$ cd lexe-public
```

Build the sidecar binary and copy it to `~/bin`

```bash
$ cargo build --release -p sdk-sidecar --bin lexe-sidecar
$ mkdir -p ~/bin
$ cp target/release/lexe-sidecar ~/bin
```

Run the sidecar binary

```bash
$ ~/bin/lexe-sidecar --help
```

Update the sidecar frequently to ensure your app continues to run smoothly:

```bash
$ cd lexe-public
$ git pull
$ cargo build --release -p sdk-sidecar --bin lexe-sidecar
$ cp target/release/lexe-sidecar ~/bin
```

See the Quickstart guide in the main [README.md](README.md) for remaining setup.
