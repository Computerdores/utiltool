use std::fmt::Display;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub pick_file_script: String,
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
