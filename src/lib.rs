pub mod config;

use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::from_utf8;

use crate::config::Config;

fn handle_utf8_error(data: &[u8]) -> Result<String, Box<dyn Error>> {
    Ok(from_utf8(data)?.to_string())
}

pub fn pick_files(cfg: &Config, initial_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let outp = Command::new("bash")
        .args(&["-c", &cfg.pick_file_script])
        .current_dir(initial_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait_with_output()?;

    if outp.status.success() {
        Ok(outp
            .stdout
            .split(|&c| c == b'\n')
            .map(|s| handle_utf8_error(s))
            .filter_map(Result::ok)
            .filter(|s| !s.is_empty())
            .filter(|s| Path::new(s).is_file())
            .collect())
    } else {
        Err(format!(
            "file picker script failed\nstderr: {:?}\nstdout: {:?}",
            handle_utf8_error(outp.stderr.as_slice())?,
            handle_utf8_error(outp.stdout.as_slice())?
        )
        .into())
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
        }
        Err(e) => Err(e),
    }
}

pub fn set_wallpaper(cfg: &Config, path: &str) -> Result<(), Box<dyn Error>> {
    run_script(&cfg.wallpaper_script, true, vec![path])?;
    Ok(())
}

fn run_script(script: &str, wait: bool, script_args: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let mut args = vec!["-c", script, "--"];
    args.extend(script_args);

    let mut command = Command::new("bash")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    if wait {
        command.wait()?;
    }

    Ok(())
}

pub fn shutdown(cfg: &Config) -> Result<(), Box<dyn Error>> {
    run_script(&cfg.system_shutdown_script, true, vec![])?;
    Ok(())
}

pub fn reboot(cfg: &Config) -> Result<(), Box<dyn Error>> {
    run_script(&cfg.system_reboot_script, true, vec![])?;
    Ok(())
}

pub fn hibernate(cfg: &Config) -> Result<(), Box<dyn Error>> {
    run_script(&cfg.system_lock_script, false, vec![])?;
    run_script(&cfg.system_hibernate_script, true, vec![])?;
    Ok(())
}

pub fn suspend(cfg: &Config) -> Result<(), Box<dyn Error>> {
    run_script(&cfg.system_lock_script, false, vec![])?;
    run_script(&cfg.system_suspend_script, true, vec![])?;
    Ok(())
}

pub fn logout(cfg: &Config) -> Result<(), Box<dyn Error>> {
    run_script(&cfg.system_logout_script, true, vec![])?;
    Ok(())
}

pub fn lock(cfg: &Config) -> Result<(), Box<dyn Error>> {
    run_script(&cfg.system_lock_script, true, vec![])?;
    Ok(())
}
