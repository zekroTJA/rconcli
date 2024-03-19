use crate::Args;
use anyhow::Result;
use config::{Config, ConfigError, File, FileFormat, Map, Source, Value, ValueKind};
use directories::ProjectDirs;
use std::{path::Path, result};

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

            if let Some(ip) = split.first() {
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

pub fn parse_config(args: &Args) -> Result<Config> {
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

    Ok(conf_builder.build()?)
}
