use clap::{command, Arg, Command};

pub fn build_cli() -> Command {
    let ssid_arg = Arg::new("SSID")
        .required(true)
        .help("The SSID of the wifi network.");
    let name_arg = Arg::new("name")
        .help("The name of the connection.");
    let autoconnect_arg = Arg::new("no-autoconnect")
        .long("no-autoconnect")
        .action(clap::ArgAction::SetTrue)
        .help("Do not automatically connect to this network.");
    let username_arg = Arg::new("username")
        .long("user")
        .help("The username for the 802.1x network.");
    command!()
        .subcommand(
            Command::new("wallpaper")
            .about("Set the wallpaper until the next reboot.")
        )
        .subcommand(
            Command::new("network")
            .about("Subcommands for easy interfacing with NetworkManager.")
            .subcommand(
                Command::new("wifi_add")
                .about("Subcommands for adding a new wifi connection.")
                .subcommand(
                    Command::new("private")
                    .about("Add a new wifi connection with WPA-PSK key management.")
                    .arg(&ssid_arg)
                    .arg(&name_arg)
                    .arg(&autoconnect_arg)
                )
                .subcommand(
                    Command::new("802.1x")
                    .about("Add a new wifi connection with 802.1x key management using PEAP and MSCHAPv2.")
                    .arg(&ssid_arg)
                    .arg(&name_arg)
                    .arg(&autoconnect_arg)
                    .arg(&username_arg)
                )
            )
            .subcommand(
                Command::new("ethernet_add")
                .about("Add a new ethernet connection with 802.1x using PEAP and MSCHAPv2.")
                .arg(&ssid_arg)
                .arg(&name_arg)
                .arg(&username_arg)
            )
            .subcommand(
                Command::new("status")
                .about("Get information about the network status.")
            )
        )
}

pub fn print_help(sub_command: &str) -> () {
    let mut base_cmd = build_cli();
    let mut cmd = Some(&mut base_cmd);

    for sub_command in (&sub_command).split_whitespace() {
        cmd = cmd.and_then(|cmd| cmd.find_subcommand_mut(sub_command));
    }
    cmd.expect(&format!("Can't print help message for non existant subcommand '{}'.", sub_command))
    .print_help().expect("Error while trying to print help message.");
}
