use std::io::{Error, ErrorKind};

pub fn preset_config_string(arg: &str) -> Result<&'static str, Error> {
    match arg.to_lowercase().as_ref() {
        "dev" => Ok(include_str!("./config.dev.toml")),
        "mining" => Ok(include_str!("./config.mining.toml")),
        "non-standard-ports" => Ok(include_str!("./config.non-standard-ports.toml")),
        "insecure" => Ok(include_str!("./config.insecure.toml")),
        "dev-insecure" => Ok(include_str!("./config.dev-insecure.toml")),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Config doesn't match any presets [dev, mining, non-standard-ports, insecure, dev-insecure]"))
    }
}
