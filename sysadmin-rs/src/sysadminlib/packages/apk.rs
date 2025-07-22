use super::*;

pub static APK: PackageManager = PackageManager {
    manager: "apk",
    list: ListArgs {
        command: &["list"],
        installed: "-I",
        available: "-a",
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
        command: &["update"],
        parser: parse_refresh,
    },
    update: UpdateArgs {
        commands: &[&["upgrade"]],
        fake: "",
        parser: parse_update,
    },
    install: InstallArgs {
        command: &["add"],
        parser: parse_install,
    },
    uninstall: UninstallArgs {
        command: &["del"],
        parser: parse_uninstall,
    },
};

fn parse_list(output: &String) -> PackagesResponse {
    let mut res = response("list");
    return res;
}

fn parse_info(output: &String) -> PackagesResponse {
    return response("info");
}

fn parse_search(output: &String) -> PackagesResponse {
    return response("search");
}

fn parse_refresh(output: &String) -> PackagesResponse {
    return response("refresh");
}

fn parse_update(output: &String) -> PackagesResponse {
    return response("update");
}

fn parse_install(output: &String) -> PackagesResponse {
    return response("install");
}

fn parse_uninstall(output: &String) -> PackagesResponse {
    return response("uninstall");
}
