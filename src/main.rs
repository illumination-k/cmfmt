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

use std::{fs, path::PathBuf};
use structopt::{clap, StructOpt};


mod settings;
mod utils;
mod fmt;

use crate::fmt::fmt;


#[derive(Debug, StructOpt)]
#[structopt(name = "mcfmt")]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(name = "markdown")]
    pub input: PathBuf,
    #[structopt(long = "config")]
    pub config: Option<PathBuf>,
    #[structopt(long = "stdout")]
    pub stdout: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let text = fs::read_to_string(&opt.input)?;
    
    let fmt_text = fmt(&text)?;

    println!("{}", fmt_text);
    Ok(())
}
