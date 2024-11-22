# Server Monitor

Server Monitor is a lightweight command-line application written in Rust that allows you to monitor the uptime of various servers and applications. It periodically pings the specified servers and displays their status in an easy-to-read table format.

## Features

- Monitors the availability of multiple servers and applications.
- Displays the current status of each server along with the last checked timestamp.
- Supports custom intervals for checking server status.
- Color-coded status indicators for active (green) and inactive (red) servers.

## Prerequisites

- Rust programming language installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- Access to the `ping` command on your operating system (available on most systems).

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/server-monitor.git
   cd server-monitor
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be located in the `target/release` directory.

## Usage

You can run the `server-monitor` application by executing the compiled binary with the desired arguments. The basic syntax is:

```bash
cargo run --servers server1,app1 server2,app2 --interval <seconds>
```

### Arguments

- `--servers` (or `-s`): A list of servers and application names in the format `server,app`. You can specify multiple pairs. If this argument is omitted, default servers will be monitored.
  
- `--interval` (or `-i`): Sets the interval in seconds for refreshing the status. The default value is 180 seconds.

### Example

To monitor Google DNS servers:

```bash
cargo run --servers 8.8.8.8,Google DNS --interval 55
```

## Default Servers

If no servers are specified, the application will monitor the following default servers:

- `chat.com` - ChatGPT Application
- `dns.google` - Google DNS Service

## Output Format

The application displays the server statuses in a table format:

```
+--------------+--------------------+----------------------+----------------------+
| Server       | App                | Result               | Last Checked         |
+--------------+--------------------+----------------------+----------------------+
| chat.com     | Chat Application   | Active ●             | 2024-11-22 14:53:59  |
| 192.4.5.11   | Example App        | Inactive ●           | 2024-11-22 14:54:10  |
| dns.google   | Google DNS Service | Active ●             | 2024-11-22 14:54:10  |
+--------------+--------------------+----------------------+----------------------+
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any modifications or improvements.

## Acknowledgements

This project makes use of the following crates:
- `clap`: for argument parsing.
- `colored`: for colored output in the terminal.
- `chrono`: for date and time handling.
- `regex`: for regular expression operations.