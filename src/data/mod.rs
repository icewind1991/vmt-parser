pub mod texture_transform;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
pub use texture_transform::TextureTransform;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Default)]
#[serde(from = "Vec2OrSingle<f32>")]
pub struct Vec2(pub [f32; 2]);

impl From<Vec2OrSingle<f32>> for Vec2 {
    fn from(value: Vec2OrSingle<f32>) -> Self {
        match value {
            Vec2OrSingle::Vec2(vec) => Vec2(vec),
            Vec2OrSingle::Single(val) => Vec2([val; 2]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Default)]
#[serde(from = "Vec3OrSingle<f32>")]
pub struct Vec3(pub [f32; 3]);

impl From<Vec3OrSingle<f32>> for Vec3 {
    fn from(value: Vec3OrSingle<f32>) -> Self {
        match value {
            Vec3OrSingle::Vec3(vec) => Vec3(vec),
            Vec3OrSingle::Single(val) => Vec3([val; 3]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Vec3OrSingle<T> {
    Vec3([T; 3]),
    Single(T),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Vec2OrSingle<T> {
    Vec2([T; 2]),
    Single(T),
}

pub(crate) fn default_scale3() -> Vec3 {
    Vec3([1.0; 3])
}

#[allow(dead_code)]
pub(crate) fn default_scale2() -> Vec2 {
    Vec2([1.0; 2])
}

pub(crate) fn default_scale() -> f32 {
    1.0
}

pub(crate) fn default_detail_scale() -> Vec2 {
    Vec2([4.0; 2])
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Copy, Clone, Default)]
#[repr(u8)]
pub enum BlendMode {
    DecalModulate = 0,
    #[default]
    Additive = 1,
    TranslucentOverlay = 2,
    BlendFactorOverlay = 3,
    TranslucentBase = 4,
    UnlitAdditive = 5,
    UnlitAdditiveThreshold = 6,
    TwoPatternModulate = 7,
    Multiply = 8,
    BaseMaskAlpha = 9,
    SelfShadowedBumpMap = 10,
    SelfShadowedBumpAlbedo = 11,
}
