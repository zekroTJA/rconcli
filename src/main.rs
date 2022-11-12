use clap::Parser;
use config::{Config, ConfigError, File, FileFormat, Map, Source, Value, ValueKind};
use directories::ProjectDirs;
use minecraft_client_rs::Client;
use std::{
    io::{self, Write},
    path::Path,
    process::exit,
    result,
};
use string_join::Join;

#[derive(Parser, Debug, Clone)]
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

impl Source for Args {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new(self.clone())
    }

    fn collect(&self) -> result::Result<Map<String, Value>, ConfigError> {
        let mut m = Map::new();
        let uri: String = "command line arguments".into();

        if let Some(password) = &self.password {
            m.insert(
                "rcon.password".into(),
                Value::new(Some(&uri), ValueKind::String(password.clone())),
            );
        }

        if let Some(address) = &self.address {
            let split: Vec<&str> = address.split(':').collect();

            if let Some(ip) = split.get(0) {
                m.insert(
                    "server-ip".into(),
                    Value::new(Some(&uri), ValueKind::String(ip.to_string())),
                );
            }

            if let Some(port) = split.get(1) {
                m.insert(
                    "rcon.port".into(),
                    Value::new(Some(&uri), ValueKind::String(port.to_string())),
                );
            }
        }

        Ok(m)
    }
}

fn main() {
    let args = Args::parse();

    // Try to load config from ./config.ini
    let mut conf_builder =
        Config::builder().add_source(File::from(Path::new("config.ini")).required(false));

    // Try to load config from $CONFIG_DIR/config.ini
    if let Some(dirs) = ProjectDirs::from("de", "zekro", "rconcli") {
        let pth = dirs
            .config_dir()
            .to_str()
            .expect("could not transform path to string");
        conf_builder =
            conf_builder.add_source(File::from(Path::new(pth).join("config.ini")).required(false));
    }

    // Load config from passed server.properties, if specified
    if let Some(properties) = &args.properties {
        conf_builder =
            conf_builder.add_source(File::new(properties, FileFormat::Ini).required(true));
    }

    // Load config from command line parameters
    conf_builder = conf_builder.add_source(args.clone());

    let conf = conf_builder
        .build()
        .expect("Failed reading config from server properties");

    let mut ip = conf.get_string("server-ip").unwrap_or_else(|_| "".into());
    if ip.is_empty() {
        ip = "localhost".into();
    }

    let mut port = conf.get_string("rcon.port").unwrap_or_else(|_| "".into());
    if port.is_empty() {
        port = "25575".into();
    }

    let addr = format!("{ip}:{port}");
    let passwd = conf
        .get_string("rcon.password")
        .expect("No password has been provided");

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
