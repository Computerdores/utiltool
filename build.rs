use std::{env, error::Error, path::PathBuf, fs::create_dir};
use clap::ValueEnum;
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let outdir = env::var("OUT_DIR")?;
    let man_dir = PathBuf::from(&outdir).join("man");
    if !man_dir.exists() {
        create_dir(&man_dir)?;
    }

    let mut cli = build_cli();
    clap_mangen::generate_to(cli.clone(), &man_dir)?;
    for &shell in Shell::value_variants() {
        generate_to(shell,&mut cli,"utiltool",&outdir)?;
    }
    
    Ok(())
}
