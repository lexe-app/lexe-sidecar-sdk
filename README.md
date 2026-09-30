# Lexe Sidecar SDK

**This repository is archived.** The Lexe Sidecar SDK is maintained in
[`lexe-app/lexe-public`](https://github.com/lexe-app/lexe-public).

- Docs: <https://docs.lexe.tech/sidecar/>
- Source code: <https://github.com/lexe-app/lexe-public/tree/master/sdk-sidecar>
- Binary releases: <https://github.com/lexe-app/lexe-public/releases>

The Lexe Sidecar SDK presents a simple JSON API for developers to control their
self-custodial, always-online [Lexe](https://lexe.app) node which can send and
receive payments over the Lightning Network. Running the `lexe-sidecar` binary
spins up a local webserver (the "sidecar") at `http://localhost:5393` which
accepts REST requests and proxies them to your Lexe node. By making simple HTTP
requests like

```
GET  http://localhost:5393/v2/node/node_info
GET  http://localhost:5393/v2/node/analyze
POST http://localhost:5393/v2/node/pay
POST http://localhost:5393/v2/node/create_invoice
POST http://localhost:5393/v2/node/pay_invoice
GET  http://localhost:5393/v2/node/payment
```

you can programmatically control your Lexe node. Your app can be written in any
language, and is portable to any environment where the `lexe-sidecar` can run.
