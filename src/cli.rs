use clap::Command;

pub fn build_cli() -> Command {
    Command::new("utiltool")
        .about("A collection of utilities")
        .subcommand(Command::new("wallpaper").about("Set the current wallpaper"))
}
