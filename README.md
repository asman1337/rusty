![Rusty Banner](rusty.png)

Rusty is a versatile command-line tool designed to assist with various tasks such as pinging hosts, downloading files, checking URL statuses, performing traceroutes, scanning ports, looking up DNS information, retrieving IP details, and more. Built in Rust for speed and reliability, Rusty is your go-to utility for troubleshooting and information gathering.

## Features

- **Ping**: Send ICMP echo requests to a host
- **Download**: Download files from the internet
- **Status Check**: Check the status of a URL
- **Traceroute**: Trace the route packets take to a network host
- **Port Scan**: Scan a host for open ports
- **DNS Lookup**: Resolve domain names to IP addresses
- **IP Info**: Retrieve information about a given IP address

We will keep adding new features to enhance Rusty's capabilities, so stay tuned for updates!

## Installation

To install Rusty, you need to have Rust and Cargo installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install Rusty by cloning the repository and building it with Cargo:

```sh
git clone https://github.com/yourusername/rusty.git
cd rusty
cargo build --release
```

After building, you can find the executable in the `target/release` directory. You can move it to a directory in your PATH for easier access:

```sh
sudo mv target/release/rusty /usr/local/bin/
```

## Usage

Rusty provides several commands to perform different network-related tasks. Below are some examples of how to use these commands:

### Ping

Send ICMP echo requests to a host:

```sh
rusty --ping 8.8.8.8 -c 10
```

### Download

Download a file from a URL:

```sh
rusty --download https://example.com/file.zip
```

### Status Check

Check the status of a URL:

```sh
rusty --status https://example.com
```

### Traceroute

Trace the route to a host:

```sh
rusty --traceroute example.com
```

### Port Scan

Scan a host for open ports:

```sh
rusty --portscan example.com
```

### DNS Lookup

Perform a DNS lookup for a domain:

```sh
rusty --dnslookup example.com
```

### IP Info

Retrieve information about an IP address:

```sh
rusty --ipinfo 8.8.8.8
```

## Contributing

We welcome contributions from the community! If you have a feature request, bug report, or would like to contribute code, please open an issue or submit a pull request on the [GitHub repository](https://github.com/asman1337/rusty).

### Adding New Features

To add new features, follow these steps:

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your feature: `git checkout -b feature-name`.
3. Implement your feature, adding new modules in the `src/commands` directory as needed.
4. Update the `Cli` struct and `Commands` enum in `src/cli.rs` to include your new command.
5. Test your feature thoroughly.
6. Commit your changes and push them to your fork: `git push origin feature-name`.
7. Open a pull request on the main repository.

## License

Rusty is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For any questions or suggestions, feel free to contact us at [asman@otmalse.in](mailto:asman@otmalse.in).

---
