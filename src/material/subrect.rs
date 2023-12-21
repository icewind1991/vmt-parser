use super::deserialize_path;
use crate::{deserialize_bare_vec2, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubRectMaterial {
    /// Base material
    #[serde(rename = "$material", deserialize_with = "deserialize_path")]
    pub material: String,
    #[serde(rename = "$pos", deserialize_with = "deserialize_bare_vec2")]
    pub pos: Vec2,
    #[serde(rename = "$size", deserialize_with = "deserialize_bare_vec2")]
    pub size: Vec2,
}
