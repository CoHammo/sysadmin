use super::*;
use crate::Exit;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::str;

mod apk;
mod apt;
mod dnf;
mod zypper;

pub struct PackageManager {
    manager: &'static str,
    list: ListArgs,
    info: InfoArgs,
    search: SearchArgs,
    refresh_data: RefreshArgs,
    update: UpdateArgs,
    install: InstallArgs,
    uninstall: UninstallArgs,
}

pub struct ListArgs {
    command: &'static [&'static str],
    installed: &'static str,
    available: &'static str,
    extras: &'static str,
    parser: fn(&String) -> PackagesResponse,
}

pub struct InfoArgs {
    command: &'static [&'static str],
    parser: fn(&String) -> PackagesResponse,
}

pub struct SearchArgs {
    command: &'static [&'static str],
    parser: fn(&String) -> PackagesResponse,
}

pub struct RefreshArgs {
    command: &'static [&'static str],
    parser: fn(&String) -> PackagesResponse,
}

pub struct UpdateArgs {
    commands: &'static [&'static [&'static str]],
    fake: &'static str,
    parser: fn(&String) -> PackagesResponse,
}

pub struct InstallArgs {
    command: &'static [&'static str],
    parser: fn(&String) -> PackagesResponse,
}

pub struct UninstallArgs {
    command: &'static [&'static str],
    parser: fn(&String) -> PackagesResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagesResponse {
    subcommand: &'static str,
    messages: Vec<String>,
    packages: Vec<Package>,
    packages_size: Option<String>,
    packages_length: Option<usize>,
    uninstalled_packages: Vec<Package>,
    uninstalled_packages_size: Option<String>,
    uninstalled_packages_length: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    arch: String,
    version: Option<String>,
    old_version: Option<String>,
    repository: Option<String>,
    size: Option<String>,
    download: Option<String>,
    description: Option<String>,
    url: Option<String>,
    license: Option<String>,
    installed: bool,
}

fn match_package_manager(os_id: &String) -> Option<&'static PackageManager> {
    match os_id.as_str() {
        "fedora" | "ultramarine" => Some(&dnf::DNF),

        "alpine" | "wolfi" => Some(&apk::APK),

        "debian" | "ubuntu" | "pop" => Some(&apt::APT),

        "opensuse-tumbleweed" | "opensuse-leap" => Some(&zypper::ZYPPER),

        &_ => None,
    }
}

pub fn match_packages_command(
    os_id: &String,
    packagescli: &PackagesArgs,
    output_type: OutputType,
) -> Exit {
    let manager: &PackageManager;
    let mut args: Vec<&str>;
    let terms: &Option<Vec<String>>;
    let fail_message: &str;
    let parser: fn(&String) -> PackagesResponse;

    match match_package_manager(os_id) {
        Some(x) => manager = x,
        None => {
            return Exit {
                exit_code: 1,
                real_command: "none",
                out: "Failed to change packages".to_string(),
                packages_response: None,
                error_message: Some(format!(
                    "This operating system ({}) is not supported",
                    os_id
                )),
            }
        }
    }

    match &packagescli.command {
        PackagesCommands::List {
            installed,
            available,
            extras,
            packages,
        } => {
            args = manager.list.command.to_vec();
            if *installed {
                args.push(&manager.list.installed)
            }
            if *available {
                args.push(&manager.list.available)
            }
            if *extras {
                args.push(&manager.list.extras)
            }
            terms = packages;
            fail_message = "Failed to list packages";
            parser = manager.list.parser;

            // let mut exit = process_command(
            //     manager.manager,
            //     args,
            //     packages,
            //     "Failed to list packages",
            //     manager.list.parser,
            // );
            // if exit.exit_code == 0 {
            //     let res = exit.packages_response.as_mut().unwrap();
            //     res.packages_length = Some(res.packages.len());
            // }
            // return exit;
        }

        PackagesCommands::Info { packages } => {
            args = manager.info.command.to_vec();
            terms = packages;
            fail_message = "Failed to get info for package(s)";
            parser = manager.info.parser;

            // let mut exit = process_command(
            //     manager.manager,
            //     manager.info.command.to_vec(),
            //     packages,
            //     "Failed to get info for package(s)",
            //     manager.info.parser,
            // );
            // if exit.exit_code == 0 {
            //     let res = exit.packages_response.as_mut().unwrap();
            //     res.packages_length = Some(res.packages.len());
            // }
            // return exit;
        }

        PackagesCommands::Search { search_terms } => {
            args = manager.search.command.to_vec();
            terms = search_terms;
            fail_message = "Failed to search for package(s)";
            parser = manager.search.parser;
        }

        PackagesCommands::Refresh => {
            args = manager.refresh_data.command.to_vec();
            terms = &None;
            fail_message = "Failed to refresh packages data";
            parser = manager.refresh_data.parser;
        }

        PackagesCommands::Update { fake } => {
            let mut exit = Exit {
                exit_code: 1,
                real_command: "none",
                error_message: Some(format!(
                    "\"update\" is not implemented for {}",
                    manager.manager
                )),
                out: "Failed to update packages".to_string(),
                packages_response: None,
            };
            for command in manager.update.commands {
                let mut commvec = command.to_vec();
                if *fake {
                    if command == manager.update.commands.last().unwrap() {
                        commvec.push(manager.update.fake);
                    }
                }
                exit = process_command(
                    manager.manager,
                    commvec,
                    &None,
                    "Failed to update packages",
                    manager.update.parser,
                    output_type,
                );
                if exit.exit_code != 0 {
                    return exit;
                }
            }
            return exit;
        }

        PackagesCommands::Install { packages } => {
            args = manager.install.command.to_vec();
            terms = packages;
            fail_message = "Failed to install package(s)";
            parser = manager.install.parser;
        }

        PackagesCommands::Uninstall { packages } => {
            args = manager.uninstall.command.to_vec();
            terms = packages;
            fail_message = "Failed to uninstall package(s)";
            parser = manager.uninstall.parser;
        }
    }

    return process_command(
        manager.manager,
        args,
        terms,
        fail_message,
        parser,
        output_type,
    );
}

fn process_command(
    package_manager: &'static str,
    args: Vec<&str>,
    terms: &Option<Vec<String>>,
    fail_message: &str,
    output_parser: fn(&String) -> PackagesResponse,
    output_type: OutputType,
) -> Exit {
    let full_command = Command::new(package_manager)
        .args(args)
        .args(terms.as_deref().unwrap_or_default())
        .output()
        .expect(fail_message);

    let code = full_command.status.code().unwrap_or(1);
    match code {
        0 => {
            match String::from_utf8(full_command.stdout.to_owned()) {
                Ok(output) => {
                    let mut response: Option<PackagesResponse> = None;
                    if output_type != OutputType::Stdout {
                        response = Some(output_parser(&output));
                    }
                    return Exit {
                        exit_code: code,
                        real_command: package_manager,
                        out: output.to_owned(),
                        packages_response: response,
                        error_message: None,
                    };
                }
                Err(_) => {
                    return Exit {
                        exit_code: code,
                        real_command: package_manager,
                        out: "No readable output".to_string(),
                        packages_response: None,
                        error_message: Some("Operation was successful, but the output could not be read from the native package manager".to_string()),
                    };
                }
            };
        }
        _ => {
            return Exit {
                exit_code: code,
                real_command: package_manager,
                out: fail_message.to_string(),
                packages_response: None,
                error_message: Some(String::from_utf8(full_command.stderr).unwrap_or_default()),
            };
        }
    }
}

pub fn response(subcommand: &'static str) -> PackagesResponse {
    return PackagesResponse {
        subcommand,
        messages: vec![],
        packages: vec![],
        packages_size: None,
        packages_length: None,
        uninstalled_packages: vec![],
        uninstalled_packages_size: None,
        uninstalled_packages_length: None,
    };
}

pub fn simplify_byte_size(num: &str) -> String {
    let mut size = num.parse::<f64>().unwrap();
    let mut unit: &str = "B";
    if size > 1024.0 {
        size /= 1024.0;
        unit = "KiB";
        if size > 1024.0 {
            size /= 1024.0;
            unit = "MiB";
            if size > 1024.0 {
                size /= 1024.0;
                unit = "GiB";
                if size > 1024.0 {
                    size /= 1024.0;
                    unit = "TiB";
                }
            }
        }
    }
    return format!("{size:.2} {unit}");
}

pub fn get_byte_size(num: &str, unit: &str) -> f64 {
    let mut size = num.parse::<f64>().unwrap();
    let multiplier: f64 = match unit {
        "B" => 1.0,
        "KiB" => 1024.0,
        "MiB" => 1048576.0,
        "GiB" => 1073741824.0,
        "TiB" => 1099511628000.0,
        _ => 0.0,
    };
    size *= multiplier;
    return size;
}
