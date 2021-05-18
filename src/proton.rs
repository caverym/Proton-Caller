use crate::config::Config;
use std::{
    ffi::OsString,
    io::{Error, ErrorKind},
     path::Path,
};

pub fn errorhere<R, T: ToString>(ek: ErrorKind, info: T) -> Result<R, Error> {
    Err(Error::new(ek, info.to_string()))
}

#[derive(Debug)]
pub struct Proton {
    proton: String,
    program: String,
    arguments: Vec<OsString>,
    config: Config,
}

impl Proton {
    pub fn new(config: Config, args: crate::AppArgs) -> Result<Proton, Error> {
        let mut proton: String = String::new();
        let program: String = args.exe;
        let arguments: Vec<OsString> = args.rest;

        if let Some(pv) = args.proton {
            proton = format!("{}/Proton {}/proton", config.common, pv);
        } else if let Some(pp) = args.custom {
            proton = format!("{}/proton", pp);
        } else {
            errorhere(ErrorKind::Other, "missing proton argument")?;
        }

        Ok(Proton {
                    proton,
                    program,
                    arguments,
                    config
                })
    }

    /// might be a dumb way of creating arguements to pass into
    /// `Command::new()`
    fn arguments(start: usize, end: usize, args: &[String], program: &str) -> Vec<String> {
        let mut vector: Vec<String> = vec![std::string::String::new(); end - (start - 2)];

        vector[0] = std::string::String::from("run");
        vector[1] = program.to_string();

        for i in start..end {
            vector[i - (start - 2)] = args[i].to_string();
        }
        vector
    }

    /// Checks if selected Proton version and Program exist. Returns
    /// `Ok<()>` on success, `Err<Error>` on failure.
    pub fn check(&self) -> Result<(), Error> {
        if !Path::new(&self.proton).exists() {
            errorhere(
                ErrorKind::NotFound,
                format!("'{}' not found", self.proton))?;
        }

        if !Path::new(&self.program).exists() {
            errorhere(ErrorKind::NotFound,
                format!("'{}' not found", self.program)
            )?;
        }

        Ok(())
    }

    /// Executes proton,,, Finally.
    pub fn execute(self) -> Result<(), Error> {
        println!("\n________Proton________");

        let log = if self.config.log {
            '1'.to_string()
        } else {
            '0'.to_string()
        };

        let mut child = std::process::Command::new(self.proton)
            .args(self.arguments)
            .env("STEAM_COMPAT_DATA_PATH", self.config.data)
            .env("PROTON_LOG", log)
            .spawn()?;

        let ecode = child.wait()?;
        if !ecode.success() {
            errorhere(
                ErrorKind::BrokenPipe,
                "Proton exited with an error",
            )?;
        }

        println!("______________________\n");
        Ok(())
    }
}
