use miette::{GraphicalReportHandler, GraphicalTheme};
use std::fs::read_to_string;
use test_case::test_case;
use vmt_parser::from_str;

#[test_case("tests/data/concretefloor003.vmt")]
#[test_case("tests/data/mvm_backpack.vmt")]
#[test_case("tests/data/water_murky.vmt")]
#[test_case("tests/data/blendrocktograss002.vmt")]
fn test_serde(path: &str) {
    let raw = read_to_string(path).unwrap();
    match from_str(&raw) {
        Ok(result) => insta::assert_ron_snapshot!(path, result),
        Err(e) => {
            let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
            let mut out = String::new();
            handler.render_report(&mut out, &e).unwrap();
            insta::assert_snapshot!(path, out)
        }
    }
}
