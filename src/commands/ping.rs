use std::net::IpAddr;
use std::time::{Duration, Instant};

use comfy_table::{Cell, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use ping::ping;

pub fn execute(host: &str, count: u32) {
    let addr: IpAddr = match host.parse() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Invalid IP address: {}", e);
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