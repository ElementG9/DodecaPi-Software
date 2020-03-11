use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub portin: u16,
    pub portout: u16,
    pub cluster: Vec<String>,
}

pub fn read_config(filename: &str) -> io::Result<Config> {
    let data = read_file(filename)?;
    let conf: Config = serde_json::from_str(&data)?;
    Ok(Config {
        portin: conf.portin,
        portout: conf.portout,
        cluster: conf.cluster,
    })
}
fn read_file(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}
