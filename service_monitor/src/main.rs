use std::{thread, time::Duration};
use colored::*;
use chrono::Local;
use clap::Parser;
use regex::Regex;
use std::str;
use std::process::Command;
use std::io::{self, Write};
use notify_rust::Notification;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ServerStatus {
    Active,
    Inactive,
    Unknown,
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

fn format_status(status: ServerStatus) -> String {
    match status {
        ServerStatus::Active => format!("Active {}", "●".green()),
        ServerStatus::Inactive => format!("Inactive {}", "●".red()),
        ServerStatus::Unknown => format!("Unknown {}", "●".yellow()),
    }
}

fn notify_inactive_servers(inactive_servers: &[(String, String)]) {
    if !inactive_servers.is_empty() {
        let message = inactive_servers
            .iter()
            .map(|(server, app)| format!("{} ({})", server, app))
            .collect::<Vec<_>>()
            .join("\n");

        Notification::new()
            .summary("Server Monitor Alert")
            .body(&format!("Inactive servers detected:\n{}", message))
            .timeout(10000) // Show notification for 10 seconds
            .show()
            .expect("Failed to show notification");
    }
}

/// Prints the table of server statuses
fn print_table(servers: &[(String, String)]) -> Vec<(String, String)> {
    let col_server_width = 15;
    let col_app_width = 20;
    let col_result_width = 25;
    let col_datetime_width = 20;
    let mut inactive_servers = Vec::new();

    println!(
        "+-----------------+----------------------+-------------------------+----------------------+"
    );
    println!(
        "| {:<col_server_width$} | {:<col_app_width$} | {:<col_result_width$} | {:<col_datetime_width$} |",
        "Server", "App", "Result", "Last Checked",
    );
    println!(
        "+-----------------+----------------------+-------------------------+----------------------+"
    );

    for (server, app) in servers {
        let status = check_server_uptime(server);
        if status == ServerStatus::Inactive || status == ServerStatus::Unknown {
            inactive_servers.push((server.clone(), app.clone()));
        }
        let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!(
            "| {:<col_server_width$} | {:<col_app_width$} | {:<col_result_width$}        | {:<col_datetime_width$} |",
            server, app, format_status(status), datetime
        );
    }

    println!(
        "+-----------------+----------------------+-------------------------+----------------------+"
    );

    inactive_servers
}

fn check_server_uptime(address: &str) -> ServerStatus {
    let ping_count_arg = if cfg!(target_os = "windows") { "-n" } else { "-c" };
    let output = Command::new("ping")
        .arg(ping_count_arg)
        .arg("1")
        .arg(address)
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            if let Ok(stdout) = str::from_utf8(&output.stdout) {
                if let Ok(re) = Regex::new(r"(\d+)% packet loss") {
                    if let Some(captures) = re.captures(stdout) {
                        if let Some(percentage) = captures.get(1) {
                            if let Ok(pct_packet_loss) = percentage.as_str().parse::<u32>() {
                                return if pct_packet_loss == 0 {
                                    ServerStatus::Active
                                } else {
                                    ServerStatus::Inactive
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    ServerStatus::Unknown
}

/// CLI structure to handle input arguments
#[derive(Parser)]
#[command(name = "Server Monitor")]
#[command(about = "Monitors servers and applications")]
struct Cli {
    /// List of servers and apps in the format: "server,app". Can specify multiple pairs.
    #[arg(short, long, value_parser = parse_server_app_pair, num_args = 1.., value_delimiter = ' ')]
    servers: Option<Vec<(String, String)>>,

    /// Interval in seconds to refresh the status
    #[arg(short, long, default_value = "180")]
    interval: u64,
}

/// Parser for server-app pairs
fn parse_server_app_pair(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() == 2 {
        Ok((parts[0].to_string(), parts[1].to_string()))
    } else {
        Err(format!("Invalid server-app pair format: '{}'. Expected 'server,app'", s))
    }
}

fn main() {
    let args = Cli::parse();

    // Default servers if none are provided
    let default_servers = vec![
        ("chat.com".to_string(), "Chat Application".to_string()),
        ("192.4.5.11".to_string(), "Example App".to_string()),
        ("dns.google".to_string(), "Google DNS Service".to_string()),
    ];

    // Use provided servers or defaults
    let servers = args.servers.unwrap_or(default_servers);

    // Monitoring loop
    loop {
        clear_terminal();
        let inactive_servers = print_table(&servers);
        notify_inactive_servers(&inactive_servers);
        thread::sleep(Duration::from_secs(args.interval));
    }
}
