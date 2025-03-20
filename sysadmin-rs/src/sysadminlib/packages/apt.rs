use super::*;

pub static APT: PackageManager = PackageManager {
    manager: "apt",
    list: ListArgs {
        command: &["list"],
        installed: "",
        available: "",
        extras: "",
        parser: parse_list,
    },
    info: InfoArgs {
        command: &["show"],
        parser: parse_info,
    },
    search: SearchArgs {
        command: &["search"],
        parser: parse_search,
    },
    refresh_data: RefreshArgs {
        command: &["update"],
        parser: parse_refresh,
    },
    update: UpdateArgs {
        commands: &[&["update"], &["upgrade"]],
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
