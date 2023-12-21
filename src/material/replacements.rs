use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vdf_reader::entry::Table;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacementsMaterial {
    pub templates: HashMap<String, ReplacementTemplate>,
    pub patterns: HashMap<String, ReplacementPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacementTemplate(pub HashMap<String, Table>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacementPattern {
    pub template: String,
}
