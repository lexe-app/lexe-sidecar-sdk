# Lexe Sidecar SDK

The Lexe Sidecar SDK presents a simple JSON API for developers to control their
self-custodial, always-online [Lexe](https://lexe.app) node which can send and
receive payments over the Lightning Network. Running the `lexe-sidecar` binary
spins up a local webserver (the "sidecar") at `http://localhost:5393` which
accepts REST requests and proxies them to your Lexe node. By making simple HTTP
requests like

```
GET  http://localhost:5393/v2/node/node_info
POST http://localhost:5393/v2/node/create_invoice
POST http://localhost:5393/v2/node/pay_invoice
GET  http://localhost:5393/v2/node/payment
```

you can programmatically control your Lexe node. Your app can be written in any
language, and is portable to any environment where the `lexe-sidecar` can run.

Install:

```bash
curl -fsSL https://lexe.app/install-sidecar.sh | sh
```

Docs:

- [Overview](https://docs.lexe.tech/sidecar/)
- [Quickstart](https://docs.lexe.tech/sidecar/quickstart/)
- [REST API Reference](https://docs.lexe.tech/sidecar/api-reference/)
- [Build from source](https://docs.lexe.tech/sidecar/build-from-source/)
- [Source code](https://github.com/lexe-app/lexe-public/tree/master/sdk-sidecar)
