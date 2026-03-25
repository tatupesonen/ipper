# ipper

A minimal service that returns your internet-facing IP address. Built with Rust and axum.

## Try it

You can look up your public IP by visiting [ipv4.narigon.dev](https://ipv4.narigon.dev) or [ipv6.narigon.dev](https://ipv6.narigon.dev) in a browser or with curl:

```sh
curl ipv4.narigon.dev
curl ipv6.narigon.dev
```

## Endpoints

| Endpoint | Response |
|----------|----------|
| `/`      | Plain text IP |
| `/json`  | `{"ip": "1.2.3.4"}` |

## Self-hosting

```sh
docker compose up -d
```

The service listens on port 3000. It supports `CF-Connecting-IP`, `X-Real-IP`, and `X-Forwarded-For` headers for correct IP detection behind reverse proxies.
