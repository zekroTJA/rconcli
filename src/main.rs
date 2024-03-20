mod cfg;
mod colors;

use anyhow::{Ok, Result};
use cfg::parse_config;
use clap::Parser;
use colors::Transformer;
use minecraft_client_rs::Client;
use std::io::{self, Write};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Location of a server.proterties file to read credentials from.
    #[clap(long)]
    properties: Option<String>,

    /// The address and port of the target server.
    #[clap(short, long)]
    address: Option<String>,

    /// The password of the target server.
    #[clap(short, long)]
    password: Option<String>,

    /// Command to execute.
    command: Vec<String>,
}

fn run() -> Result<()> {
    let args = Args::parse();

    let conf = parse_config(&args)?;

    let ip = conf
        .get_string("server-ip")
        .unwrap_or_else(|_| "localhost".into());

    let port = conf
        .get_string("rcon.port")
        .unwrap_or_else(|_| "25575".into());

    let addr = format!("{ip}:{port}");
    let passwd = conf
        .get_string("rcon.password")
        .or_else(|_| prompt_password())?;

    let mut client =
        Client::new(addr).map_err(|e| anyhow::anyhow!("failed initializing client: {e}"))?;
    client
        .authenticate(passwd)
        .map_err(|e| anyhow::anyhow!("authentication failed: {e}"))?;

    let mut transformer = Transformer::new();

    if !args.command.is_empty() {
        let cmd = args.command.join(" ");
        if cmd.is_empty() {
            anyhow::bail!("empty command");
        }

        let res = client
            .send_command(cmd)
            .map_err(|e| anyhow::anyhow!("command execution failed: {e}"))?;
        println!("{}", transformer.transform(&res.body));
        return Ok(());
    }

    let mut cmd = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        cmd.clear();
        io::stdin().read_line(&mut cmd)?;

        if cmd.is_empty() {
            continue;
        }

        let cmd = cmd.trim();

        match cmd.to_lowercase().as_str() {
            "exit" | "e" | "quit" | "q" => return Ok(()),
            _ => {}
        }

        let res = client
            .send_command(cmd.into())
            .map_err(|e| anyhow::anyhow!("command execution failed: {e}"))?;

        println!("{}", transformer.transform(&res.body));
    }
}

fn prompt_password() -> Result<String> {
    print!("password: ");
    std::io::stdout().flush()?;
    let passwd = rpassword::read_password()?;
    Ok(passwd)
}

fn main() {
    run().expect("failed")
}
