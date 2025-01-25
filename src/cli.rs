use clap::{Command, command};

pub fn build_cli() -> Command {
    command!()
        .subcommand(
            Command::new("wallpaper")
            .about("Set the current wallpaper (resets on reboot).")
        )
}
