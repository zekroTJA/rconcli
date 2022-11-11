use std::{
    io::{self, Write},
    process::exit,
};

use clap::Parser;

use config::{Config, File, FileFormat};
use minecraft_client_rs::Client;
use string_join::Join;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
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

fn main() {
    let args = Args::parse();

    let mut addr = "localhost:25575".to_string();
    let mut passwd: Option<String> = None;

    if let Some(properties) = args.properties {
        let conf = Config::builder()
            .add_source(File::new(&properties, FileFormat::Ini).required(true))
            .build()
            .expect("Failed reading config from server properties");

        let ip = conf
            .get_string("server-ip")
            .unwrap_or_else(|_| "localhost".into());
        let port = conf
            .get_string("rcon.port")
            .unwrap_or_else(|_| "25575".into());
        addr = format!("{ip}:{port}");

        passwd = conf.get_string("rcon.password").ok();
    }

    if let Some(arg_addr) = args.address {
        addr = arg_addr;
    }

    if let Some(arg_passwd) = args.password {
        passwd = Some(arg_passwd);
    }

    let passwd = passwd.expect("No password has been provided");

    let mut client = Client::new(addr).expect("Failed connecting to server");
    client.authenticate(passwd).expect("Authentication failed");

    let cmd = " ".join(args.command);

    if !cmd.is_empty() {
        let res = client.send_command(cmd).expect("Command execution failed");
        println!("{}", res.body);
        exit(0);
    }

    loop {
        let mut cmd = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Reading from STDIN failed");

        if cmd.is_empty() {
            continue;
        }

        let res = client
            .send_command(cmd[..cmd.len() - 1].into())
            .expect("Command execution failed");

        println!("{}", res.body);
    }
}
