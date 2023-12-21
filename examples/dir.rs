use miette::{Context, IntoDiagnostic, Result};
use std::env::args;
use std::fs::read_to_string;
use std::path::Path;
use vmt_parser::from_str;
use vmt_parser::material::Material;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let mut success = 0;
    let mut err = Vec::new();
    let dir = args().nth(1).expect("no path provided");
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap_or_default().ends_with(".vmt"))
    {
        if let Err(e) = try_parse(entry.path()) {
            err.push((entry.path().to_path_buf(), e));
            let e = try_parse(entry.path()).unwrap_err();
            println!("{:?}", e);
        } else {
            success += 1;
            println!("{}", entry.path().display());
        }
    }

    println!("successfully parsed {success} files");
    println!("found errors in {} files", err.len());
    for (path, e) in err {
        println!("{}: {e:?}", path.display());
    }

    Ok(())
}

fn try_parse(path: &Path) -> Result<Material> {
    let raw = read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;
    from_str(&raw).into_diagnostic()
}
