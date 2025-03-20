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
    // let mut lines = output.lines();

    // let sections: Vec<&str>;
    // while !lines.next().unwrap_or("Installed packages") {}
    // let line = lines.next().unwrap_or_default();
    // if !line.is_empty() {
    //     line
    // }

    // let out_split: Vec<&str> = sections_re.split(output.as_str()).collect();
    // println!(
    //     "INSTALLED PACKAGES:\n{}\n\nAVAILABLE PACKAGES:\n{}\n\nEXTRA PACKAGES:\n{}",
    //     out_split[0], out_split[1], out_split[2]
    // );

    //===========================================
    // This is a split between new and old code
    //===========================================

    // let re = Regex::new(r"\n\S").unwrap();
    // let parts = re.split(output.as_str());

    // let mut lines = output.split("~~~");
    // if output.contains("Repositories loaded.") {
    //     res.messages.push("Updated repos"));
    //     while !lines.next().unwrap().contains("Repositories loaded.") {}
    // }
    // let mut installed = false;
    // let mut previous: &str = "";
    // let mut byte_size: f64 = 0.0;
    // loop {
    //     let line = lines.next().unwrap_or_default();
    //     if line.is_empty() {
    //         break;
    //     }
    //     let mut parts: Vec<&str> = line.split("---").collect();
    //     parts[0] = parts[0].trim_matches('"');
    //     if (previous == parts[0]) && installed {
    //         let i = res.packages.len() - 1;
    //         res.packages[i].repository = Some(parts[3]));
    //         res.packages[i].download = Some(simplify_byte_size(parts[5]));
    //     } else {
    //         if parts[3] == "@System" {
    //             installed = true;
    //         } else {
    //             installed = false;
    //         }
    //         let mut description: Option<String> = None;
    //         let mut url: Option<String> = None;
    //         let mut license: Option<String> = None;
    //         if parts.len() == 9 {
    //             description = Some(parts[6]));
    //             url = Some(parts[7]));
    //             license = Some(parts[8]));
    //         }
    //         byte_size += parts[4].parse::<f64>().unwrap();
    //         res.packages.push(Package {
    //             name: parts[0]),
    //             arch: parts[1]),
    //             version: Some(parts[2])),
    //             old_version: None,
    //             repository: Some(parts[3])),
    //             size: Some(simplify_byte_size(parts[4])),
    //             download: Some(simplify_byte_size(parts[5])),
    //             description,
    //             url,
    //             license,
    //             installed,
    //         });
    //     }
    //     previous = parts[0];
    // }
    // res.packages_size = Some(simplify_byte_size(byte_size.to_string().as_str()));
    return res;
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
