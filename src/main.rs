#![warn(clippy::all, clippy::pedantic)]
mod proton;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let proton: proton::Proton;
    let custom: bool;

    if args.len() == 1 {
        missing_args();
        return;
    }

    match args[1].as_str() {
        "--help" | "-h" => {
            help();
            return;
        }
        "--version" | "-v" => {
            pc_version();
            return;
        }
        "--setup" | "-s" => {
            setup();
            return;
        }
        "--custom" | "-c" => custom = true,
        _ => custom = false,
    }

    match proton::Proton::init(&args, custom) {
        Ok(s) => proton = s,
        Err(e) => {
            eprintln!("proton-call: {}", e);
            return;
        }
    }

    if let Err(e) = proton.execute() {
        eprintln!("proton-call: {}", e);
    }
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
    println!("  proton-caller 2.2.0 Copyright (C) 2021  Avery Murray");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions.\n")
}

fn missing_args() {
    println!("proton-call: missing arguments");
    println!("Try 'proton-call --help' for more information");
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
