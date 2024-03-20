mod cfg;
mod colors;

use anyhow::Result;
use cfg::parse_config;
use clap::Parser;
use colors::Transformer;
use is_terminal::IsTerminal;
use minecraft_client_rs::Client;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{
    io::{self, Write},
    process::exit,
};
use yansi::Paint;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Command to execute.
    command: Vec<String>,

    /// Location of a server.proterties file to read credentials from.
    #[arg(long)]
    properties: Option<String>,

    /// The address and port of the target server.
    #[arg(short, long)]
    address: Option<String>,

    /// The password of the target server.
    #[arg(short, long)]
    password: Option<String>,

    /// Supress colored output
    #[arg(long)]
    no_color: bool,
}

fn run() -> Result<()> {
    let args = Args::parse();

    let enable_color = io::stdout().is_terminal() && !args.no_color;
    match enable_color {
        true => yansi::enable(),
        false => yansi::disable(),
    }

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

    let mut client = Client::new(addr.clone())
        .map_err(|e| anyhow::anyhow!("failed initializing client: {e}"))?;
    client
        .authenticate(passwd)
        .map_err(|e| anyhow::anyhow!("authentication failed: {e}"))?;

    let mut transformer = Transformer::new(!enable_color);

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

    let mut rl = DefaultEditor::new()?;

    println!(
        "{} {}\n{} {} {} {} {} {} {}\n",
        "RCON CLI connected to".bold().dim(),
        addr.bold().dim(),
        "You can use".dim(),
        "help".yellow().italic(),
        "to list available commands or use".dim(),
        "exit".cyan().italic(),
        "or".dim(),
        "quit".cyan().italic(),
        "to exit.".dim(),
    );

    let prompt = "> ".dim().to_string();
    loop {
        let cmd = match rl.readline(&prompt) {
            Ok(cmd) => cmd,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        match cmd.to_lowercase().as_str() {
            "exit" | "e" | "quit" | "q" => return Ok(()),
            _ => {}
        }

        let res = client
            .send_command(cmd.clone())
            .map_err(|e| anyhow::anyhow!("command execution failed: {e}"))?;

        rl.add_history_entry(cmd)?;

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
    if let Err(err) = run() {
        writeln!(io::stderr(), "{}: {}", "error".red().bold(), err).expect("writing error");
        exit(1);
    }
}
