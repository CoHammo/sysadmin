use super::*;

pub static ZYPPER: PackageManager = PackageManager {
    manager: "dnf",
    list: ListArgs {
        command: &["list"],
        installed: "--installed-only",
        available: "--not-installed-only",
        extras: "",
        parser: parse_list,
    },
    info: InfoArgs {
        command: &["info"],
        parser: parse_info,
    },
    search: SearchArgs {
        command: &["search"],
        parser: parse_search,
    },
    refresh_data: RefreshArgs {
        command: &["refresh"],
        parser: parse_refresh,
    },
    update: UpdateArgs {
        commands: &[&["refresh"], &["update"]],
        fake: "",
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

fn parse_list(output: &String) -> PackagesResponse {
    todo!();
}

fn parse_info(output: &String) -> PackagesResponse {
    todo!();
}

fn parse_search(output: &String) -> PackagesResponse {
    todo!();
}

fn parse_refresh(output: &String) -> PackagesResponse {
    todo!();
}

fn parse_update(output: &String) -> PackagesResponse {
    todo!();
}

fn parse_install(output: &String) -> PackagesResponse {
    todo!();
}

fn parse_uninstall(output: &String) -> PackagesResponse {
    todo!();
}
