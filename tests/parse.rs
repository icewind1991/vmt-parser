use miette::{GraphicalReportHandler, GraphicalTheme};
use std::fs::read_to_string;
use test_case::test_case;
use thiserror::Error;
use vmt_parser::from_str;
use vmt_parser::material::Material;

#[derive(Debug, Error)]
enum LoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Vdf(#[from] vdf_reader::VdfError),
}

#[test_case("tests/data/concretefloor003.vmt")]
#[test_case("tests/data/mvm_backpack.vmt")]
#[test_case("tests/data/water_murky.vmt")]
#[test_case("tests/data/blendrocktograss002.vmt")]
#[test_case("tests/data/patch.vmt")]
#[test_case("tests/data/handrail128_skin2.vmt")]
#[test_case("tests/data/shot2_subrect.vmt")]
fn test_serde(path: &str) {
    let raw = read_to_string(path).unwrap();
    match from_str(&raw) {
        Ok(result) => {
            insta::assert_ron_snapshot!(path, result);
            if let Material::Patch(patch) = result {
                let patched = patch
                    .resolve(|path| read_to_string(path).map_err(LoaderError::from))
                    .unwrap();
                insta::assert_ron_snapshot!(format!("{}_resolved", path), patched);
            }
        }
        Err(e) => {
            let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
            let mut out = String::new();
            handler.render_report(&mut out, &e).unwrap();
            insta::assert_snapshot!(path, out)
        }
    }
}
