use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version = "0.1.0",
    author = "Asman Mirza",
    about = "Rusty: A multi-purpose utility tool",
    long_about = "Rusty is a versatile command-line tool designed to assist with various tasks such as pinging hosts, downloading files, checking URL statuses, performing traceroutes, scanning ports, looking up DNS information, retrieving IP details and so on."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "--ping", about = "Send ICMP echo requests to a host")]
    Ping {
        #[arg(value_name = "host")]
        host: String,
        #[arg(short, long, default_value_t = 4)]
        count: u32,
    },
    #[command(name = "--download", about = "Download a file from a URL")]
    Download {
        #[arg(value_name = "url")]
        url: String,
    },
    #[command(name = "--status", about = "Check the status of a URL")]
    StatusCheck {
        #[arg(value_name = "url")]
        url: String,
    },
    #[command(name = "--traceroute", about = "Trace the route to a host")]
    Traceroute {
        #[arg(value_name = "host")]
        host: String,
    },
    #[command(name = "--portscan", about = "Scan a host for open ports")]
    PortScan {
        #[arg(value_name = "host")]
        host: String,
    },
    #[command(name = "--dnslookup", about = "Perform a DNS lookup for a domain")]
    DnsLookup {
        #[arg(value_name = "domain")]
        domain: String,
    },
    #[command(name = "--ipinfo", about = "Retrieve information about an IP address")]
    IpInfo {
        #[arg(value_name = "ip")]
        ip: String,
    },
}