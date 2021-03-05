extern crate anyhow;
extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;
extern crate toml;

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[macro_use]
extern crate serde;

use anyhow::Result;
use utils::write_string;

use std::{
    env::var,
    fs,
    path::{Path, PathBuf},
    process::exit,
};
use structopt::{clap, StructOpt};

mod fmt;
mod settings;
mod utils;

use crate::fmt::fmt;
use crate::settings::{read_settings, write_default_settings, Settings};

#[derive(Debug, StructOpt)]
#[structopt(name = "mcfmt")]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(
        name = "markdown",
        help = "Path of the input markdown you would like to format"
    )]
    pub input: PathBuf,
    #[structopt(
        long = "config",
        help = "Path of the config file. default: ${home}/.config/cmfmt.toml"
    )]
    pub config: Option<PathBuf>,
    #[structopt(
        long = "stdout",
        help = "Output formatted markdown to stdout instead of overwrite the input markdown"
    )]
    pub stdout: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let settings: Settings = match &opt.config {
        Some(config) => read_settings(config)?,
        None => {
            let home = match var("HOME") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Cannot find $HOME: {}", e);
                    exit(1)
                }
            };

            let config_path = Path::new(&home).join(".config/cmfmt.toml");

            if !config_path.exists() {
                eprintln!("write default settings in {:?}", config_path);
                write_default_settings(&config_path)?;
            }

            read_settings(&config_path)?
        }
    };

    let text = fs::read_to_string(&opt.input)?;
    let fmt_text = fmt(&text, &settings)?;

    if (&opt.stdout).to_owned() {
        println!("{}", fmt_text);
    } else {
        write_string(&opt.input, &fmt_text)?;
    }

    Ok(())
}
