use std::error::Error;
use std::process::{Command, Stdio};
use std::str::from_utf8;

fn handle_utf8_error(data: &[u8]) -> Result<String, Box<dyn Error>> {
    Ok(from_utf8(data)?.to_string())
}

pub fn file_picker(initial_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let outp = Command::new("nnn")
        .args(&["-a", "-p", "-", initial_dir])
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait_with_output()?;
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
