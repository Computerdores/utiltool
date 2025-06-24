mod cli;

use utiltool::config::Config;

fn set_wallpaper(cfg: &Config) {
    let result = utiltool::pick_file(cfg, "/etc/nixos/common/wallpapers");
    if let Ok(path) = result {
        utiltool::set_wallpaper(&path).unwrap();
    } else {
        eprintln!("{}", result.unwrap_err());
    }
}

fn main() {
    let args = cli::build_cli().get_matches();

    let config = Config::read();

    match args.subcommand() {
        Some(("wallpaper", _)) => set_wallpaper(&config),
        Some(("system", sys_args)) => match sys_args.subcommand() {
            Some(("shutdown", _)) => utiltool::shutdown(&config).unwrap(),
            Some(("reboot", _)) => utiltool::reboot(&config).unwrap(),
            Some(("hibernate", _)) => utiltool::hibernate(&config).unwrap(),
            Some(("suspend", _)) => utiltool::suspend(&config).unwrap(),
            Some(("logout", _)) => utiltool::logout(&config).unwrap(),
            Some(("lock", _)) => utiltool::lock(&config).unwrap(),
            None => cli::print_help("system"),
            _ => (),
        },
        None => cli::print_help(""),
        _ => (),
    }
}
