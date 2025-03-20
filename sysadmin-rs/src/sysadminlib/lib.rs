use clap::{Parser, Subcommand, ValueEnum};
use packages::PackagesResponse;
use serde::{Deserialize, Serialize};
use std::fs;

pub mod packages;

//========================================
// Commands and arguments for the program
//========================================

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommands,

    #[arg(value_enum, short, long, default_value = "stdout")]
    pub output: OutputType,
}

#[derive(Subcommand)]
pub enum CliCommands {
    Packages(PackagesArgs),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputType {
    Stdout,
    Json,
    Msgpack,
}

// Arguments and commands for "packages" subcommand
//==================================================

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct PackagesArgs {
    #[command(subcommand)]
    pub command: PackagesCommands,
}

#[derive(Subcommand)]
pub enum PackagesCommands {
    List {
        #[arg(short, long)]
        installed: bool,

        #[arg(short, long)]
        available: bool,

        #[arg(short, long)]
        extras: bool,

        packages: Option<Vec<String>>,
    },
    Info {
        packages: Option<Vec<String>>,
    },
    Search {
        terms: Option<Vec<String>>,
    },
    Refresh,
    Update {
        #[arg(short, long)]
        fake: bool,
    },
    Install {
        packages: Option<Vec<String>>,
    },
    Uninstall {
        packages: Option<Vec<String>>,
    },
}

//========================================
// Helper structs and methods
//========================================

/// Struct to store exit info from sysadmin commands
#[derive(Serialize, Deserialize, Debug)]
pub struct Exit {
    pub exit_code: i32,
    real_command: &'static str,
    pub out: String,
    packages_response: Option<PackagesResponse>,
    error_message: Option<String>,
}

/// Gets the OS from "/etc/os-release" on Linux distributions
pub fn get_os() -> String {
    let contents =
        fs::read_to_string("/etc/os-release").expect("Could not determine the operating system");
    let lines = contents.lines();
    for line in lines {
        let value: Vec<&str> = line.split("=").collect();
        if value[0] == "ID" {
            return value[1].to_string();
        }
    }
    return "none".to_string();
}
