use clap::{command, Command};

pub fn build_cli() -> Command {
    command!()
        .subcommand(Command::new("wallpaper").about("Set the wallpaper until the next reboot."))
}

pub fn print_help(sub_command: &str) -> () {
    let mut base_cmd = build_cli();
    let mut cmd = Some(&mut base_cmd);

    for sub_command in (&sub_command).split_whitespace() {
        cmd = cmd.and_then(|cmd| cmd.find_subcommand_mut(sub_command));
    }
    cmd.expect(&format!(
        "Can't print help message for non existant subcommand '{}'.",
        sub_command
    ))
    .print_help()
    .expect("Error while trying to print help message.");
}
