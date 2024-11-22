use std::{thread, time::Duration};
use colored::*;
use chrono::Local;
use clap::Parser;
use regex::Regex;
use std::str;
use std::process::Command;
use std::io::{self, Write};

fn clear_terminal() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

fn format_status(is_active: Option<f32>) -> String {
    match is_active {
        Some(0.0) => format!("Active {}", "●".green()),
        Some(_) => format!("Inactive {}", "●".red()),
        None => format!("Inactive {}", "●".red()),
    }
}

/// Prints the table of server statuses
fn print_table(servers: &[(String, String)]) {
    let col_server_width = 12;
    let col_app_width = 18;
    let col_result_width = 20;
    let col_datetime_width = 20;

    println!(
        "+--------------+--------------------+----------------------+----------------------+"
    );
    println!(
        "| {:<col_server_width$} | {:<col_app_width$} | {:<col_result_width$} | {:<col_datetime_width$} |",
        "Server", "App", "Result", "Last Checked",
    );
    println!(
        "+--------------+--------------------+----------------------+----------------------+"
    );

    for (server, app) in servers {
        let status = format_status(check_server_uptime(server));
        let datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!(
            "| {:<col_server_width$} | {:<col_app_width$} | {:<col_result_width$}          | {:<col_datetime_width$} |",
            server, app, status, datetime
        );
    }

    println!(
        "+--------------+--------------------+----------------------+----------------------+"
    );
}

/// Simulates fucntion for testing
// pub fn check_server_uptime(_address: &str) -> Option<f32> {
//     Some(0.0) // Placeholder for actual server-checking logic
// }

pub fn check_server_uptime(address: &str) -> Option<f32> {
    let ping_count_arg = if cfg!(target_os = "windows") { "-n" } else { "-c" };
    let output = Command::new("ping")
        .arg(ping_count_arg)
        .arg("1")
        .arg(address)
        .output()
        .expect("Ping command failed to start");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).ok()?;
        let re = Regex::new(r"(\d+\.?\d*)% packet loss").ok()?;

        if let Some(captures) = re.captures(stdout) {
            if let Some(percentage) = captures.get(1) {
                return percentage.as_str().parse::<f32>().ok();
            }
        }
    }
    None
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
        print_table(&servers);
        thread::sleep(Duration::from_secs(args.interval));
    }
}