use clap::{command, Command};

pub fn build_cli() -> Command {
    command!()
        .subcommand(Command::new("wallpaper").about("Set the wallpaper until the next reboot."))
        .subcommand(
            Command::new("system")
                .about("Trigger various system actions.")
                .subcommand(Command::new("shutdown").about("Shutdown the system."))
                .subcommand(Command::new("reboot").about("Reboot the system."))
                .subcommand(Command::new("hibernate").about("Send the system into hibernation."))
                .subcommand(Command::new("suspend").about("Suspend the system."))
                .subcommand(
                    Command::new("logout").about("Logout out of the current wayland session."),
                )
                .subcommand(Command::new("lock").about("Lock the current wayland session.")),
        )
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
