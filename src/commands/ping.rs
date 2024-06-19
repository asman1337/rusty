use std::net::{IpAddr, ToSocketAddrs};
use std::time::{Duration, Instant};
use comfy_table::{Cell, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use ping::ping;

pub fn execute(host: &str, count: u32) {
    let addr = match resolve_host(host) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to resolve host {}: {}", host, e);
            return;
        }
    };

    let timeout = Some(Duration::from_secs(5));
    let ttl = None;
    let ident = None;
    let seq_cnt = Some(1);
    let payload = None;

    let mut table = Table::new();
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec![
        Cell::new("Seq"),
        Cell::new("Host"),
        Cell::new("Time (ms)"),
    ]);

    for seq in 1..=count {
        let start = Instant::now();
        match ping(addr, timeout, ttl, ident, seq_cnt, payload) {
            Ok(_) => {
                let duration = start.elapsed();
                table.add_row(vec![
                    Cell::new(seq.to_string()),
                    Cell::new(host),
                    Cell::new(duration.as_millis().to_string()),
                ]);
            }
            Err(e) => {
                eprintln!("Failed to ping {}: {}", host, e);
                table.add_row(vec![
                    Cell::new(seq.to_string()),
                    Cell::new(host),
                    Cell::new("Failed"),
                ]);
            }
        }
    }

    println!("{}", table);
}

fn resolve_host(host: &str) -> Result<IpAddr, String> {
    let addrs = match (host, 0).to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(e) => return Err(format!("Failed to resolve host: {}", e)),
    };

    addrs.filter_map(|addr| match addr {
        std::net::SocketAddr::V4(v4) => Some(IpAddr::V4(v4.ip().clone())),
        std::net::SocketAddr::V6(v6) => Some(IpAddr::V6(v6.ip().clone())),
    })
        .next()
        .ok_or_else(|| "No valid IP address found".to_string())
}
