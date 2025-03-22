use regex::Regex;
use std::{ptr::null, str::Lines};

use super::*;

pub static DNF: PackageManager = PackageManager {
    manager: "dnf",
    list: ListArgs {
        command: &["list"],
        installed: "--installed",
        available: "--available",
        extras: "--extras",
        parser: parse_list,
    },
    info: InfoArgs {
        // command: &[
        //     "repoquery",
        //     "--queryformat=%{name}---%{arch}---%{version}---%{repoid}---%{installsize}---%{downloadsize}---%{description}---%{url}---%{license}~~~",
        //     "--installed",
        //     "--available",
        // ],
        command: &["info"],
        parser: parse_info,
    },
    search: SearchArgs {
        command: &["search"],
        parser: parse_search,
    },
    refresh_data: RefreshArgs {
        command: &["check-upgrade"],
        parser: parse_refresh,
    },
    update: UpdateArgs {
        commands: &[&["upgrade"]],
        fake: "--downloadonly",
        parser: parse_update,
    },
    install: InstallArgs {
        command: &["install"],
        parser: parse_install,
    },
    uninstall: UninstallArgs {
        command: &["remove"],
        parser: parse_uninstall,
    },
};

/// Parse list output
fn parse_list(output: &String) -> PackagesResponse {
    let mut res = response("list");
    let mut lines;
    if output.contains("Repositories loaded.") {
        res.messages.push("Updated repos".to_string());
    }

    let stops = ["Installed packages", "Available packages", "Extra packages"];
    for stop in stops {
        lines = output.lines();
        while !lines.next().unwrap_or(stop).contains(stop) {}
        loop {
            let line = lines.next().unwrap_or_default();
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            let first_parts: Vec<&str> = parts[0].split(".").collect();
            let installed = stop != "Available packages";
            res.packages.push(Package {
                name: first_parts[0].to_string(),
                arch: first_parts[1].to_string(),
                version: Some(parts[1].to_string()),
                old_version: None,
                repository: Some(parts[2].to_string()),
                size: None,
                download: None,
                description: None,
                url: None,
                license: None,
                installed,
            })
        }
    }
    res.packages_length = Some(res.packages.len());
    return res;
}

/// Parse info output
fn parse_info(output: &String) -> PackagesResponse {
    let mut res = response("info");

    let package_split = Regex::new(r"\n\n").unwrap();
    let field_split = Regex::new(r"\n\S").unwrap();
    let parts = package_split.split(output.as_str());
    let mut installed = false;
    for mut part in parts {
        if part.starts_with("Installed packages\n") {
            installed = true;
            part = &part[19..];
        }
        if part.starts_with("Available packages\n") {
            installed = false;
            part = &part[19..]
        }
        let fields: Vec<&str> = field_split.split(part).collect();
        if installed {
            res.packages.push(Package {
                name: clean(fields[0]),
                arch: clean(fields[4]),
                version: Some(clean(fields[2])),
                old_version: None,
                repository: Some(clean(fields[7])),
                size: Some(clean(fields[5])),
                download: None,
                description: Some(clean(fields[11])),
                url: Some(clean(fields[9])),
                license: Some(clean(fields[10])),
                installed,
            });
        } else {
            res.packages.push(Package {
                name: clean(fields[0]),
                arch: clean(fields[4]),
                version: Some(clean(fields[2])),
                old_version: None,
                repository: Some(clean(fields[8])),
                size: Some(clean(fields[6])),
                download: Some(clean(fields[5])),
                description: Some(clean(fields[12])),
                url: Some(clean(fields[10])),
                license: Some(clean(fields[11])),
                installed,
            });
        }
    }

    return res;
}

fn clean(to_clean: &str) -> String {
    let clean = Regex::new(r"[\S ]*: | +:").unwrap();
    return clean.replace_all(to_clean, "").to_string();
}

/// Parse search output
fn parse_search(output: &String) -> PackagesResponse {
    let mut res = response("search");
    let mut lines = output.lines();
    if output.contains("Repositories loaded.") {
        res.messages.push("Updated repos".to_string());
        while !lines.next().unwrap().contains("Repositories loaded.") {}
    }
    if output.contains("No matches found.") {
        res.messages.push("Nothing matched search".to_string());
        return res;
    }
    loop {
        let line = lines.next().unwrap_or_default();
        if line.is_empty() {
            break;
        }
        if line.contains("Matched fields:") {
            continue;
        }
        let parts: Vec<&str> = line.split(":").collect();
        let pack: Vec<&str> = parts[0].split(".").collect();
        res.packages.push(Package {
            name: pack[0].trim_start().to_string(),
            arch: pack[1].to_string(),
            version: None,
            old_version: None,
            repository: None,
            size: None,
            download: None,
            description: Some(parts[1].trim_start().to_string()),
            url: None,
            license: None,
            installed: false,
        });
    }
    res.packages_length = Some(res.packages.len());
    return res;
}

/// Parse refresh output
fn parse_refresh(_output: &String) -> PackagesResponse {
    let mut res = response("refresh");
    res.messages.push("Updated repos".to_string());
    return res;
}

/// Parse update output
fn parse_update(output: &String) -> PackagesResponse {
    let mut res = response("update");
    if output.contains("Repositories loaded.") {
        res.messages.push("Updated repos".to_string());
    }
    if output.contains("Nothing to do.") {
        res.messages.push("No updates available".to_string());
        return res;
    }
    let mut lines = output.lines();
    let mut installed_byte_size: f64 = 0.0;
    let mut uninstalled_byte_size: f64 = 0.0;

    // Loop to parse "Removing" section of packages
    while !lines.next().unwrap().contains("Removing:") {}
    loop {
        let line = lines.next().unwrap_or_default();
        if line.contains("Removing dependent packages:") {
            continue;
        }
        if !line.starts_with(" ") {
            break;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        uninstalled_byte_size += get_byte_size(parts[4], parts[5]);
        res.uninstalled_packages.push(Package {
            name: parts[0].to_string(),
            arch: parts[1].to_string(),
            version: Some(parts[2].to_string()),
            old_version: None,
            repository: Some(parts[3].to_string()),
            size: Some(format!("{} {}", parts[4], parts[5])),
            download: None,
            description: None,
            url: None,
            license: None,
            installed: false,
        })
    }

    // Loop to parse "Upgrading" section of packages
    lines = output.lines();
    while !lines.next().unwrap().contains("Upgrading:") {}
    loop {
        let line = lines.next().unwrap_or_default();
        if line.contains("Installing:") {
            continue;
        }
        if !line.starts_with(" ") {
            break;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "replacing" {
            let i = res.packages.len() - 1;
            res.packages[i].old_version = Some(parts[3].to_string());
            uninstalled_byte_size += get_byte_size(parts[5], parts[6]);
        } else {
            installed_byte_size += get_byte_size(parts[4], parts[5]);
            res.packages.push(Package {
                name: parts[0].to_string(),
                arch: parts[1].to_string(),
                version: Some(parts[2].to_string()),
                old_version: None,
                repository: Some(parts[3].to_string()),
                size: Some(format!("{} {}", parts[4], parts[5])),
                download: None,
                description: None,
                url: None,
                license: None,
                installed: true,
            })
        }
    }

    let uninstalled_size = simplify_byte_size(uninstalled_byte_size.to_string().as_str());
    let installed_size = simplify_byte_size(installed_byte_size.to_string().as_str());
    res.uninstalled_packages_size = Some(uninstalled_size);
    res.packages_size = Some(installed_size);
    res.packages_length = Some(res.packages.len());
    res.uninstalled_packages_length = Some(res.uninstalled_packages.len());
    return res;
}

/// Parse install output
fn parse_install(output: &String) -> PackagesResponse {
    let mut res = response("install");
    if output.contains("Repositories loaded.") {
        res.messages.push("Updated repos".to_string());
    }
    if output.contains("Nothing to do.") {
        res.messages
            .push("Package(s) already installed".to_string());
        return res;
    }
    let mut lines = output.lines();
    while !lines.next().unwrap().contains("Installing:") {}

    let mut byte_size: f64 = 0.0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        if line.contains("Installing dependencies:") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        byte_size += get_byte_size(parts[4], parts[5]);
        res.packages.push(Package {
            name: parts[0].to_string(),
            arch: parts[1].to_string(),
            version: Some(parts[2].to_string()),
            old_version: None,
            repository: Some(parts[3].to_string()),
            size: Some(format!("{} {}", parts[4], parts[5])),
            download: None,
            description: None,
            url: None,
            license: None,
            installed: true,
        });
    }
    res.packages_size = Some(simplify_byte_size(byte_size.to_string().as_str()));
    res.packages_length = Some(res.packages.len());
    return res;
}

/// Parse uninstall output
fn parse_uninstall(output: &String) -> PackagesResponse {
    let mut res = response("uninstall");
    if output.contains("Nothing to do.") {
        res.messages.push("No packages to uninstall".to_string());
        return res;
    }
    let mut lines = output.lines();
    while !lines.next().unwrap().contains("Removing:") {}

    let mut byte_size: f64 = 0.0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        if line.contains("Removing unused dependencies:") {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        byte_size += get_byte_size(parts[4], parts[5]);
        res.uninstalled_packages.push(Package {
            name: parts[0].to_string(),
            arch: parts[1].to_string(),
            version: Some(parts[2].to_string()),
            old_version: None,
            repository: Some(parts[3].to_string()),
            size: Some(format!("{} {}", parts[4], parts[5])),
            download: None,
            description: None,
            url: None,
            license: None,
            installed: true,
        });
    }
    res.uninstalled_packages_size = Some(simplify_byte_size(byte_size.to_string().as_str()));
    res.uninstalled_packages_length = Some(res.uninstalled_packages.len());
    return res;
}
