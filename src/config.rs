use std::fmt::Display;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_values::pick_file_script")]
    pub pick_file_script: String,

    #[serde(default = "default_values::wallpaper_script")]
    pub wallpaper_script: String,

    #[serde(default = "default_values::system_shutdown_script")]
    pub system_shutdown_script: String,
    #[serde(default = "default_values::system_reboot_script")]
    pub system_reboot_script: String,
    #[serde(default = "default_values::system_hibernate_script")]
    pub system_hibernate_script: String,
    #[serde(default = "default_values::system_suspend_script")]
    pub system_suspend_script: String,
    #[serde(default = "default_values::system_logout_script")]
    pub system_logout_script: String,
    #[serde(default = "default_values::system_lock_script")]
    pub system_lock_script: String,
}

pub mod default_values {
    pub fn pick_file_script() -> String {
        r"
        t=$(mktemp -t 'utiltool-pick-file.XXXXXX')
        yazi --chooser-file=$t
        cat $t
        rm $t
        "
        .to_string()
    }

    pub fn wallpaper_script() -> String {
        r"
        hyprctl hyprpaper preload $1 >/dev/null
        hyprctl hyprpaper wallpaper ,$1 >/dev/null
        "
        .to_string()
    }

    pub fn system_shutdown_script() -> String {
        "shutdown now".to_string()
    }

    pub fn system_reboot_script() -> String {
        "reboot".to_string()
    }

    pub fn system_hibernate_script() -> String {
        "systemctl hibernate".to_string()
    }

    pub fn system_suspend_script() -> String {
        "systemctl suspend".to_string()
    }

    pub fn system_logout_script() -> String {
        "uwsm stop".to_string()
    }

    pub fn system_lock_script() -> String {
        "hyprlock --immediate >/dev/null".to_string()
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Config {{ pick_file_script: {} }}",
            self.pick_file_script
        )
    }
}

impl Config {
    pub fn read() -> Config {
        let path = xdg::BaseDirectories::with_prefix("utiltool")
            .find_config_file("config.yaml")
            .unwrap();
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path.to_str().unwrap()))
            .build()
            .expect("Failed to read config file.");
        settings
            .try_deserialize::<Config>()
            .expect("Failed to parse config.")
    }
}
