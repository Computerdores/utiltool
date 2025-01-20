use std::env;

fn print_usage() {
    println!("Usage: {} <command>", env::args().nth(0).unwrap());
    println!("Commands:");
    println!("  help, -h, --help: Print this help message");
    println!("  wallpaper: Set the current wallpaper");
}

fn set_wallpaper() {
    let result = utiltool::file_picker("/etc/nixos/common/wallpapers");
    if let Ok(paths) = result {
        if paths.len() > 0 {
            for path in paths {
                println!("Picked file: {}", path);
            }
        } else {
            println!("No file picked");
        }
    } else {
        eprintln!("{}", result.unwrap_err());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "help" | "-h" | "--help" => print_usage(),
        "wallpaper" => set_wallpaper(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}
