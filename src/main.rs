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
        println!("{}", HELP);
        exit(0);
    } else if pargs.contains(["-v", "--version"]) {
        pc_version();
        exit(0);
    } else if pargs.contains(["-s", "--setup"]) {
        setup();
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

fn pc_version() {
    println!("\t{} {} Copyright (C) 2021 {}", CRATE, VERSION, AUTHOR);
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions.")
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

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const CRATE: &str = env!("CARGO_PKG_NAME");

const HELP: &str = "\
USAGE: proton-call [-p, -e, -c, -h, -s, -v]

FLAGS:
\t-c, --custom [PATH]\tUse Proton from PATH
\t-e, --exe [EXECUTABLE]\tpath to Windows executable to use execute
\t-p, --proton [VERSION]\tuse Proton VERSION, uses latest if not used
\t-h, --help\t\tdisplay this help information
\t-s, --setup\t\tdisplay setup information
\t-v, --version\t\tdisplay version information
";