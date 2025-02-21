mod cli;

fn set_wallpaper() {
    let result = utiltool::pick_file("/etc/nixos/common/wallpapers");
    if let Ok(path) = result {
        utiltool::set_wallpaper(&path).unwrap();
    } else {
        eprintln!("{}", result.unwrap_err());
    }
}

fn main() {
    let args = cli::build_cli().get_matches();

    match args.subcommand() {
        Some(("wallpaper", _)) => set_wallpaper(),
        None => cli::print_help(""),
        _ => (),
    }
}
