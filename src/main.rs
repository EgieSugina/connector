use std::env;
use std::process::{Command, Stdio};
use regex::Regex;

fn display_usage() {
    println!("SASA Bypass SOCKSv5 Port 1080 by Errorgeist.id");
    println!("Usage: connector.exe user@server.com");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        display_usage();
        return;
    }

    let input = &args[1];
    let re = Regex::new(r"(\w+)@(\S+)").unwrap();

    if let Some(caps) = re.captures(input) {
        let user = &caps[1];
        let server = &caps[2];

        let ssh_command = format!("ssh -D 1080 -o ServerAliveInterval=60 -o ServerAliveCountMax=5 {}@{}", user, server);

        println!("Executing command: {}", ssh_command);

        let shell = if cfg!(target_os = "windows") { "cmd" } else { "sh" };
        let shell_arg = if cfg!(target_os = "windows") { "/C" } else { "-c" };

        let status = Command::new(shell)
            .arg(shell_arg)
            .arg(ssh_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        let output = status.wait_with_output().expect("Failed to read stdout");

        if output.status.success() {
            println!("Successfully connected to SSH proxy.");
            println!("To use the proxy, set your browser to use a SOCKS5 proxy at localhost:1080.");
        } else {
            eprintln!("Failed to establish SSH proxy.");
        }
    } else {
        eprintln!("Invalid input format. Use user@server.com");
    }
}
