use super::deserialize_path;
use crate::{default_scale, default_scale3, TextureTransform, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterMaterial {
    /// Usually referred to as a "sludge-layer", acts as a layer on top of the surface of the $AboveWater Material.
    #[serde(rename = "$basetexture", deserialize_with = "deserialize_path")]
    pub base_texture: String,
    /// Tells this material is used for models and not brushes.
    #[serde(rename = "$abovewater", default)]
    pub above_water: bool,
    /// Required parameter. This is the material (not texture) to use when underneath the water’s surface. The bottom material must have $reflecttexture, $abovewater and $envmap disabled, but can otherwise do whatever it wants.
    #[serde(
        rename = "$bottommaterial",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub bottom_material: Option<String>,
    /// Applies a refracting screen overlay when the camera is underwater. Generally used with effects\water_warp01. Requires $abovewater to be 0.
    #[serde(
        rename = "$underwaterover",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub underwater_overlay: Option<String>,
    /// Specifies a texture that will provide three-dimensional lighting information for a material for DX8.
    #[serde(rename = "$bumpmap", default, deserialize_with = "deserialize_path")]
    pub bump_map: Option<String>,
    /// A Du/dv map for DirectX 8 rendering ($bumpmap), and a bump map for DirectX 9 and above ($normalmap).
    #[serde(rename = "$normalmap", default, deserialize_with = "deserialize_path")]
    pub normal_map: Option<String>,
    #[serde(rename = "$dudvframe", default)]
    pub du_dv_frame: u32,
    /// Frame to start the animated du/dv map and bump map on, respectively
    #[serde(rename = "$bumpframe", default)]
    pub dump_frame: u32,
    /// Transforms the bump map texture.
    #[serde(rename = "$bumptransform", default)]
    pub bump_transform: TextureTransform,
    /// Tints the results of projected textures on the water. The flashlight mainly affects the brigthness of the fog in the water.
    #[serde(rename = "$flashlighttint", default = "default_scale")]
    pub flash_light_tint: f32,

    /// Enable volumetric fog for the water.
    #[serde(rename = "$fogenable", default)]
    pub fog_enable: bool,
    /// Color of the water’s volumetric fog. Generally this value should match the color used in the bottom material.
    #[serde(rename = "$fogenable", default = "default_scale3")]
    pub fog_color: Vec3,
    /// Distance in units/inches from the eye at which water fog starts.
    #[serde(rename = "$fogstart", default)]
    pub fog_start: f32,
    /// Distance in units/inches from the eye at which water fog ends.
    #[serde(rename = "$fogend", default)]
    pub fog_end: f32,
    /// Enable volumetric fog for the water.
    #[serde(rename = "$lightmapwaterfog", default)]
    pub light_map_water_fog: bool,
    // todo: https://developer.valvesoftware.com/wiki/Water_(shader) from reflection onward
}
