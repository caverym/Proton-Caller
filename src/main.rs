#![warn(clippy::all, clippy::pedantic)]
mod config;
mod proton;

use config::Config;
use proton::Proton;

use std::io::{Result as IoR, ErrorKind};
use std::{ffi::OsString, process::exit};

use crate::proton::errorhere;

const PROTON_LATEST: &str = "6";

#[macro_export]
macro_rules! eprinter {
    ($p:expr, $e:expr) => {
        eprintln!("{}: error: {}", $p, $e);
    };
}

#[derive(Debug)]
pub struct AppArgs {
    proton: Option<String>,
    custom: Option<String>,
    exe: String,
    rest: Vec<OsString>,
    full_args: Vec<String>,
    full_args_count: usize,
}

fn main() -> IoR<()> {

    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => errorhere(ErrorKind::Other, e)?,
    };

    let config: Config = Config::new()?;

    println!("{:#?}", args);

    let proton = Proton::new(config, args)?;

    println!("{:#?}", proton);

    proton.check()?;
    proton.execute()?;

    Ok(())
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let full_args: Vec<String> = std::env::args().collect();
    let full_args_count: usize = full_args.len();

    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        help();
        exit(0);
    }

    let args = AppArgs {
        proton: pargs.opt_value_from_str(["-p", "--proton"]).unwrap_or(Some(PROTON_LATEST.to_string())),
        custom: pargs.opt_value_from_str(["-c", "--custom"])?,
        exe: pargs.value_from_str(["-e", "--exe"])?,
        rest: pargs.finish(),
        full_args,
        full_args_count,
    };

    Ok(args)
}

// messaging
fn help() {
    println!("Usage: proton-call VERSION PROGRAM");
    println!("   or: basename OPTION PATH PROGRAM");
    println!("Execute PROGRAM with Proton VERSION");
    println!("If specified, run proton PATH\n");
    println!("  -c, --custom PATH       use proton from PATH");
    println!("  -h, --help              display this help message");
    println!("  -s, --setup             display setup information");
    println!("  -v, --version           display version information");
}

fn pc_version() {
    println!("  proton-caller 2.2.4 Copyright (C) 2021  Avery Murray");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions.\n")
}

fn setup() {
    println!("Configuration of proton-call requires a config file located at");
    println!("`~/.config/proton.conf` which is formatted like:\n");
    println!("  data = \"\"");
    println!("  common = \"\"\n");
    println!("`data` is used to give proton a directory used for compatibility data.\n");
    println!("`common` is a directory pointing to steam's common directory, where Proton");
    println!("and games are installed");
}
