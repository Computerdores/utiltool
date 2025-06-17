pub mod config;

use std::error::Error;
use std::process::{Command, Stdio};
use std::str::from_utf8;

use crate::config::Config;

fn create_temp_file() -> Result<String, Box<dyn Error>> {
    let temp_file = tempfile::NamedTempFile::new()?;
    let path = temp_file.path().to_str().ok_or("Failed to convert path to string")?;
    Ok(path.to_string())
}

fn delete_temp_file(path: &str) -> Result<(), Box<dyn Error>> {
    std::fs::remove_file(path).map_err(|e| e.into())
}

fn handle_utf8_error(data: &[u8]) -> Result<String, Box<dyn Error>> {
    Ok(from_utf8(data)?.to_string())
}

pub fn pick_files(cfg: &Config, initial_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let temp_file = create_temp_file()?;
    std::fs::write(&temp_file, cfg.pick_file_script.as_bytes())?;

    let outp = Command::new("bash")
        .args(&[&temp_file])
        .current_dir(initial_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait_with_output()?;

    delete_temp_file(temp_file.as_str())?;

    if outp.status.success() {
        Ok(
            outp.stdout.split(|&c| c == b'\n')
            .map(|s| handle_utf8_error(s))
            .filter_map(Result::ok)
            .filter(|s| s.len() > 0)
            .collect()
        )
    } else {
        Err(
            format!(
                "nnn failed\nstderr: {:?}\nstdout: {:?}",
                handle_utf8_error(outp.stderr.as_slice())?,
                handle_utf8_error(outp.stdout.as_slice())?
            ).into()
        )
    }
}

pub fn pick_file(cfg: &Config, initial_dir: &str) -> Result<String, Box<dyn Error>> {
    match pick_files(cfg, initial_dir) {
        Ok(mut files) => {
            if files.len() == 1 {
                Ok(files.remove(0))
            } else if files.len() > 1 {
                Err("More than one file picked".into()) // TODO handle this case
            } else {
                Err("No file picked".into())
            }
        },
        Err(e) => Err(e),
    }
}

pub fn set_wallpaper(path: &str) -> Result<(), Box<dyn Error>> {
    Command::new("hyprctl")
        .args(&["hyprpaper", "preload", path])
        .spawn()?
        .wait()?;
    Command::new("hyprctl")
        .args(&["hyprpaper", "wallpaper", format!(",{}", path).as_str()])
        .spawn()?
        .wait()?;
    Ok(())
}