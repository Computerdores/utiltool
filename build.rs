use std::{env, error::Error};
use clap::ValueEnum;
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> Result<(), Box<dyn Error>> {
    let outdir = env::var("OUT_DIR")?;

    let mut cli = build_cli();
    for &shell in Shell::value_variants() {
        generate_to(shell,&mut cli,"utiltool",&outdir)?;
    }
    
    Ok(())
}
