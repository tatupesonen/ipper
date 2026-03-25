use std::net::{IpAddr, SocketAddr};

use axum::{Json, Router, extract::ConnectInfo, http::HeaderMap, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct IpResponse {
    ip: String,
}

fn client_ip(headers: &HeaderMap, addr: SocketAddr) -> IpAddr {
    let ip = headers
        .get("x-real-ip")
        .or_else(|| headers.get("x-forwarded-for"))
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
        .unwrap_or_else(|| addr.ip());

    // Normalize IPv4-mapped IPv6 (::ffff:1.2.3.4) to plain IPv4
    match ip {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map(IpAddr::V4).unwrap_or(ip),
        _ => ip,
    }
}

async fn handle(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Json<IpResponse> {
    let ip = client_ip(&headers, addr);
    Json(IpResponse {
        ip: ip.to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("server error");
}
