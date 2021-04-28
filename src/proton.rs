use crate::config;
pub(crate) struct Proton {
    proton: String,
    arguments: Vec<String>,
    conf: config::Config,
}

impl Proton {
    pub fn init(args: &[String], custom: bool) -> Result<Proton, &'static str> {
        let mut start: usize = 3;
        if custom {
            return Proton::init_custom(&args);
        }
        if if_arg(&args[1]) {
            return Err("error: invalid argument");
        }
        let args_len: usize = args.len();
        if args_len < 2 {
            return Err("error: not enough arguments");
        }
        
        let config: config::Config;
        let version: String = args[1].to_string();
        let program: String;
        let path: String;

        match config::Config::new() {
            Ok(val) => config = val,
            Err(e) => return Err(e),
        }

        if let Ok(val) = Proton::locate_proton(&version, &config.common) {
            path = val;
            program = args[2].to_string();
        } else {
            match Proton::locate_proton("5.13", &config.common) {
                Ok(val) => {
                    path = val;
                }
                Err(e) => return Err(e),
            }
            program = args[1].to_string();
            start = 2;
        }

        if !Proton::check([&path, &program].to_vec()) {
            return Err("error: invalid Proton or executable");
        }

        let a: Vec<String> = Proton::arguments(start, args_len, &args, &program);
        println!("Proton:   {}", path.split('/').last().unwrap());
        println!("Program:  {}", program.split('/').last().unwrap());

        Ok(Proton {
            proton: format!("{}/proton", path),
            arguments: a,
            conf: config,
        })
    }

    fn arguments(start: usize, end: usize, args: &[String], program: &str) -> Vec<String> {
        let mut vector: Vec<String> = vec![std::string::String::new(); end - (start - 2)];
        vector[0] = std::string::String::from("run");
        vector[1] = program.to_string();

        for i in start..end {
            vector[i - (start - 2)] = args[i].to_string();
        }
        vector
    }

    fn check(file: Vec<&String>) -> bool {
        for i in file {
            if !std::path::Path::new(i).exists() {
                return false;
            }
        }
        true
    }

    fn locate_proton(version: &str, common: &str) -> Result<String, &'static str> {
        let dir: std::fs::ReadDir;
        match std::fs::read_dir(common) {
            Ok(val) => dir = val,
            Err(_) => return Err("error: can not find common directory"),
        }

        for path in dir {
            let p = path.unwrap().path();
            let d = p.to_str().unwrap();
            if d.contains(version) {
                return Ok(d.to_string());
            }
        }
        Err("error: invalid Proton version")
    }

    fn init_custom(args: &[String]) -> Result<Proton, &'static str> {
        let config: config::Config;
        let args_len: usize = args.len();
        if args_len < 4 {
            return Err("error: not enough arguments");
        }

        let path: String = args[2].to_string();

        if !Proton::check([&path, &args[3]].to_vec()) {
            return Err("error: invalid Proton or executable");
        }

        let a: Vec<String> = Proton::arguments(4, args_len, &args, &args[3]);

        match config::Config::new() {
            Ok(val) => config = val,
            Err(e) => return Err(e),
        }

        println!("Proton:   custom");
        println!("Program:  {}", args[3].split('/').last().unwrap());

        Ok(Proton {
            proton: format!("{}/proton", path),
            arguments: a,
            conf: config,
        })
    }

    pub fn execute(self) -> Result<(), &'static str> {
        let ecode: std::process::ExitStatus;
        let mut child: std::process::Child;
        println!("\n________Proton________");

        match std::process::Command::new(self.proton)
            .args(self.arguments)
            .env("STEAM_COMPAT_DATA_PATH", self.conf.data)
            .spawn()
        {
            Ok(val) => child = val,
            Err(_) => return Err("error: failed to launch Proton"),
        }

        match child.wait() {
            Ok(val) => ecode = val,
            Err(_) => return Err("error: failed to wait for Proton"),
        }

        if !ecode.success() {
            return Err("error: Proton exited with an error");
        }
        println!("______________________\n");
        Ok(())
    }
}

fn if_arg(the_arg: &str) -> bool {
    let arg: Vec<char> = the_arg.chars().collect();
    if arg[0] == '-' {
        return true;
    }
    false
}
