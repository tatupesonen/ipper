use std::net::{IpAddr, SocketAddr};

use axum::{Json, Router, extract::ConnectInfo, http::HeaderMap, middleware, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct IpResponse {
    ip: String,
}

fn client_ip(headers: &HeaderMap, addr: SocketAddr) -> IpAddr {
    let ip = headers
        .get("cf-connecting-ip")
        .or_else(|| headers.get("x-real-ip"))
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

async fn log_request(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: middleware::Next,
) -> axum::response::Response {
    let ip = client_ip(&headers, addr);
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let response = next.run(request).await;
    println!("{} {} {} {}", ip, method, path, response.status().as_u16());
    response
}

async fn handle_text(headers: HeaderMap, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    client_ip(&headers, addr).to_string()
}

async fn handle_json(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Json<IpResponse> {
    let ip = client_ip(&headers, addr);
    Json(IpResponse { ip: ip.to_string() })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_text))
        .route("/json", get(handle_json))
        .layer(middleware::from_fn(log_request));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("server error");
}
