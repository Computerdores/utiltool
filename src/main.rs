use clap::Command;

fn set_wallpaper() {
    let result = utiltool::pick_file("/etc/nixos/common/wallpapers");
    if let Ok(path) = result {
        utiltool::set_wallpaper(&path).unwrap();
    } else {
        eprintln!("{}", result.unwrap_err());
    }
}

fn main() {
    let args =  Command::new("utiltool")
        .about("A collection of utilities")
        .subcommand(Command::new("wallpaper").about("Set the current wallpaper"))
        .get_matches();

    match args.subcommand() {
        Some(("wallpaper", _)) => set_wallpaper(),
        _ => (),
    }
}
