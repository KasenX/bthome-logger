use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub database_url: url::Url,
}

pub fn read<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}
