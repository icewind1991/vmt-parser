use miette::{Context, IntoDiagnostic, Result};
use std::env::args;
use std::fs::read_to_string;
use vmt_parser::from_str;

fn main() -> Result<()> {
    let path = args().nth(1).expect("no path provided");
    let raw = read_to_string(path)
        .into_diagnostic()
        .wrap_err("failed to read input")?;
    let material = from_str(&raw).wrap_err("failed to parse material")?;
    dbg!(material);
    Ok(())
}
