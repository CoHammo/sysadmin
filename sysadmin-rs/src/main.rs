use clap::Parser;
use rmp_serde::Serializer;
use serde::{self, Serialize};
use std::io::{self, Write};
use sysadminlib::packages::match_packages_command;
use sysadminlib::{get_os, CliCommands, OutputType};

fn main() -> Result<(), i32> {
    let commands = sysadminlib::CliArgs::parse();
    let os_id = get_os();

    let exit = match &commands.command {
        CliCommands::Packages(packagescli) => {
            match_packages_command(&os_id, packagescli, commands.output)
        }
    };

    match &commands.output {
        OutputType::Stdout => println!("{}", exit.out),
        OutputType::Json => println!("{}", serde_json::to_string_pretty(&exit).unwrap()),
        OutputType::Msgpack => {
            let mut buf = Vec::new();
            let mut s = Serializer::new(&mut buf);
            exit.serialize(&mut s).unwrap();
            io::stdout().write_all(&buf).unwrap_or_default();
        }
    }

    return match exit.exit_code {
        0 => Ok(()),
        _ => Err(exit.exit_code),
    };
}
