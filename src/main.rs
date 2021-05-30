#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

/*!
# Proton Caller

Run any Windows program through [Valve's Proton](https://github.com/ValveSoftware/Proton).

## Usage:

Defaults to the latest version of Proton.
```
proton-call -r foo.exe
```

Defaults to the latest verson of Proton, all extra arguments passed to the executable.
```
proton-call -r foo.exe --flags --for program
```

Uses specified version of Proton, any extra arguments will be passed to the executable.
```
proton-call -p 5.13 -r foor.exe
```

Uses custom version of Proton, give the past to directory, not the Proton executable itself.
```
proton-call -c '/path/to/Proton version' -r foo.exe
```
*/

use std::ffi::OsString;
use std::io::{Error, ErrorKind, Read, Result};
use std::result::Result as DualRes;

static PROTON_LATEST: &str = "6.3";

struct Proton {
    proton: String,
    executable: String,
    passed_args: Vec<OsString>,
    config: Config,
    args: Args,
}

impl Proton {
    pub fn new(config: Config, args: Args) -> Result<Proton> {
        let mut proton = Proton {
            proton: String::new(),
            executable: String::new(),
            passed_args: Vec::new(),
            config,
            args,
        };

        proton.set_proton();
        proton.set_executable();
        proton.pass_arguments();

        Ok(proton)
    }

    fn set_proton(&mut self) {
        if let Some(c) = &self.args.custom {
            self.proton = format!("{}/proton", c);
        } else {
            let proton: String = self
                .args
                .proton
                .clone()
                .unwrap_or_else(|| PROTON_LATEST.to_string());
            self.proton = format!("{}/Proton {}/proton", self.config.common, proton);
        }
        vprintln!(self.args.verbose, "setting proton: {}", self.proton)
    }

    fn set_executable(&mut self) {
        self.executable = self.args.program.clone();
        vprintln!(self.args.verbose, "setting executable: {}", self.executable);
    }

    fn pass_arguments(&mut self) {
        self.passed_args = self.args.extra.clone();
    }

    pub fn check(&self) -> Result<()> {
        use std::path::Path;

        if !Path::new(&self.proton).exists() {
            error!(ErrorKind::NotFound, "{} not found!", self.proton)?;
        }

        if !Path::new(&self.executable).exists() {
            error!(ErrorKind::NotFound, "{} not found!", self.executable)?;
        }

        Ok(())
    }

    pub fn run(self) -> Result<()> {
        use std::process::{Child, Command};

        let log = if self.args.log { "1" } else { "0" };

        vprintln!(
            self.args.verbose,
            "Running:
        Command:    '{}',
        Exe:        '{}' {:?},
        Environment:    PROTON_LOG={}, STEAM_COMPAT_DATA_PATH={}
        ",
            self.proton,
            self.executable,
            self.passed_args,
            log,
            self.config.data
        );

        println!("\n________Proton________");

        let mut child: Child = Command::new(self.proton)
            .arg("run")
            .arg(self.executable)
            .args(self.passed_args)
            .env("PROTON_LOG", log)
            .env("STEAM_COMPAT_DATA_PATH", self.config.data)
            .spawn()?;

        let exitcode = child.wait()?;
        println!("______________________\n");

        vprintln!(self.args.verbose, "exited with value {}", exitcode);

        Ok(())
    }
}

/// Stores needed data for PC configuration.
#[derive(serde_derive::Deserialize)]
struct Config {
    data: String,
    common: String,
}

impl Config {
    /// Function to load in the user's config file.
    pub fn load(v: bool) -> Result<Config> {
        use std::fs::File;
        let mut file: File;

        let path: String = Config::get_config_path()?;

        vprintln!(v, "reading config from: {}", path);

        file = File::open(path)?;

        if file.metadata().is_err() {
            error!(ErrorKind::NotFound, "Did not find config file!")?;
        }

        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;

        let config_dat: String = match String::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => error!(ErrorKind::Other, "{}", e)?,
        };

        let config: Config = toml::from_str(&config_dat)?;

        vprintln!(v, "successfully read config file");

        Ok(config)
    }

    /// Finds a config file and returns the expected config location.
    fn get_config_path() -> Result<String> {
        use std::env::var;

        if let Ok(val) = var("XDG_CONFIG_HOME") {
            Ok(format!("{}/proton.conf", val))
        } else if let Ok(val) = var("HOME") {
            Ok(format!("{}/.config/proton.conf", val))
        } else {
            error!(ErrorKind::Other, "Failed to read environment!")
        }
    }
}

/// Stores and parses arguments for controlling PC.
struct Args {
    proton: Option<String>,
    custom: Option<String>,
    program: String,
    log: bool,
    verbose: bool,
    extra: Vec<OsString>,
}

impl Args {
    /// Parses the arguments.
    pub fn args() -> DualRes<Args, pico_args::Error> {
        use pico_args::Arguments;
        use std::process::exit;

        let mut pargs: Arguments = Arguments::from_env();

        if pargs.contains(["-h", "--help"]) {
            println!("{}", HELP);
            exit(0);
        } else if pargs.contains(["-V", "--version"]) {
            version();
            exit(0);
        }

        let args: Args = Args {
            proton: pargs.opt_value_from_str(["-p", "--proton"])?,
            custom: pargs.opt_value_from_str(["-c", "--custom"])?,
            program: pargs.value_from_str(["-r", "--run"])?,
            log: pargs.contains(["-l", "--log"]),
            verbose: pargs.contains(["-v", "--verbose"]),
            extra: pargs.finish(),
        };

        Ok(args)
    }
}

/// Main function exists only to handle errors.
fn main() {
    use std::process::exit;
    if let Err(e) = proton_caller() {
        eprintln!("{}error:{} {}", lliw::Fg::Red, lliw::Fg::Reset, e);
        exit(1);
    }
}

/// The effective main function of the program.
fn proton_caller() -> Result<()> {
    // Gather arguments.
    let args: Args = match Args::args() {
        Ok(a) => a,
        Err(e) => error!(ErrorKind::Other, "{}", e)?,
    };

    let config: Config = Config::load(args.verbose)?;

    let proton: Proton = Proton::new(config, args)?;

    proton.check()?;

    proton.run()?;

    Ok(())
}

/// Macro to run the `error_here` function.
#[macro_export]
macro_rules! error {
    ($ek:expr, $fmt:expr) => { error_here($ek, $fmt) };
    ($ek:expr, $fmt:literal, $($arg:expr),*) => { error_here($ek, format!($fmt, $($arg),*)) }
}

/// Quick and dirt way to cause an error in a function.
fn error_here<R, T: ToString>(ek: ErrorKind, info: T) -> Result<R> {
    Err(Error::new(ek, info.to_string()))
}

/// Verbose println macro, checks `$v` to be true, if is, prints the text.
#[macro_export]
macro_rules! vprintln {
    ($v:expr, $fmt:literal, $($arg:expr),*) => {
        if $v { println!($fmt, $($arg),*) }
    };

    ($v:expr, $fmt:literal) => {
        if $v { println!($fmt) }
    }
}

fn version() {
    println!("Proton Caller (proton-call) {} Copyright (C) 2021 {}",
             env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
}

static HELP: &str = "\
Usage: proton-call [OPTIONS]... EXE [EXTRA]...

Options:
    -c, --custom [PATH]     Path to a directory containing Proton to use
    -h, --help              View this help message
    -l, --log               Pass PROTON_LOG variable to Proton
    -p, --proton [VERSION]  Use Proton VERSION from `common`
    -r, --run EXE           Run EXE in proton
    -v, --verbose           Run in verbose mode
    -V, --version           View version information

Config:
    The config file should be located at '$XDG_CONFIG_HOME/proton.conf' or '$HOME/.config/proton.conf'

    The config requires two values.
    Data: a location to any directory to contain Proton's runtime files.
    Common: the directory where your proton versions are stored, usually Steams' common directory.

    Example:
        data = \"/home/avery/Documents/Proton/env/\"
        common = \"/home/avery/.steam/steam/steamapps/common/\"
";
