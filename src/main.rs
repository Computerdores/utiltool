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
        Some(("network", sub_m)) => {
            match sub_m.subcommand() {
                Some(("wifi_add", sub_m)) => {
                    match sub_m.subcommand() {
                        Some(("private", _sub_m)) => {
                            panic!("not implemented");
                        },
                        Some(("802.1x", _sub_m)) => {
                            panic!("not implemented");
                        },
                        Some((cmd, _)) => panic!("Unknown subcommand 'network wifi_add {}'.", cmd),
                        None => cli::print_help("network wifi_add"),
                    }
                }
                Some(("ethernet_add", _sub_m)) => {
                    panic!("not implemented");
                },
                Some(("status", _)) => {
                    panic!("not implemented");
                },
                Some((cmd, _)) => panic!("Unknown subcommand 'network {}'.", cmd),
                None => cli::print_help("network"),
            }
        }
        Some((cmd, _)) => panic!("Unknown subcommand '{}'.", cmd),
        None => cli::print_help(""),
    }
}
