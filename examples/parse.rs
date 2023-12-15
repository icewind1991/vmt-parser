use miette::{Context, IntoDiagnostic, Result};
use std::env::args;
use std::fs::read_to_string;
use vdf_reader::Reader;
use vmt_parser::material::Material;

fn main() -> Result<()> {
    let path = args().nth(1).expect("no path provided");
    let raw = read_to_string(path)
        .into_diagnostic()
        .wrap_err("failed to read input")?;
    let mut reader = Reader::from(raw.as_str());
    let material = Material::parse(&mut reader).wrap_err("failed to parse material")?;
    dbg!(material);
    Ok(())
}
