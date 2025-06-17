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
        None => cli::print_help(""),
        _ => (),
    }
}
