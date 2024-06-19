use clap::Parser;

use cli::Cli;
use commands::{dns_lookup, download, ip_info, ping, port_scan, status_check, traceroute};

use crate::cli::Commands;

mod cli;
mod commands;

const BANNER: &str = r#"
 ______    __  __   ______   _________  __  __
/_____/\  /_/\/_/\ /_____/\ /________/\/_/\/_/\
\:::_ \ \ \:\ \:\ \\::::_\/_\__.::.__\/\ \ \ \ \
 \:(_) ) )_\:\ \:\ \\:\/___/\  \::\ \   \:\_\ \ \
  \: __ `\ \\:\ \:\ \\_::._\:\  \::\ \   \::::_\/
   \ \ `\ \ \\:\_\:\ \ /____\:\  \::\ \    \::\ \
    \_\/ \_\/ \_____\/ \_____\/   \__\/     \__\/
"#;

fn main() {
    println!("{}", BANNER);


    let cli = Cli::parse();

    match &cli.command {
        Commands::Ping { host } => {
            ping::execute(host);
        }
        Commands::Download { url } => {
            download::execute(url);
        }
        Commands::StatusCheck { url } => {
            status_check::execute(url);
        }
        Commands::Traceroute { host } => {
            traceroute::execute(host);
        }
        Commands::PortScan { host } => {
            port_scan::execute(host);
        }
        Commands::DnsLookup { domain } => {
            dns_lookup::execute(domain);
        }
        Commands::IpInfo { ip } => {
            ip_info::execute(ip);
        }
    }
}