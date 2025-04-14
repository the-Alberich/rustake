use std::net::SocketAddr;

pub fn get_app_addr() -> SocketAddr {
    let port = std::env::var("APP_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("APP_PORT must be a valid u16");

    SocketAddr::from(([127, 0, 0, 1], port))
}
