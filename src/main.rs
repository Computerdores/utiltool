fn main() {
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
