use std::{path::PathBuf, sync::LazyLock};

use eyre::Result;
use pico_args::Arguments;

pub struct Config {
    pub path: PathBuf,
}

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| parse_args(&mut Arguments::from_env()).expect("could not parse config"));

fn parse_args(args: &mut Arguments) -> Result<Config> {
    Ok(Config {
        path: args.free_from_str()?,
    })
}
