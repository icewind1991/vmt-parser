use super::deserialize_path;
use crate::{default_scale, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeRefractMaterial {
    /// Usually referred to as a "sludge-layer", acts as a layer on top of the surface of the $AboveWater Material.
    #[serde(rename = "$iris", deserialize_with = "deserialize_path")]
    pub iris: String,
    /// A texture to specify the shape of the cornea, similar to a normal map. Influences lighting and dilation. The red and green channels are used for the normal mapping, the blue channel is a mask for parallax mapping (straight multiply), and the alpha channel seems to be a multiplier for lighting. Because the $iris is warped in various ways the normals will not match 1-to-1 with the base texture.
    #[serde(rename = "$corneatexture", deserialize_with = "deserialize_path")]
    pub cornea_texture: String,
    /// Strength of the $corneatexture.
    #[serde(rename = "$corneabumpstrength", default = "default_scale")]
    pub cornea_bump_strength: f32,
    /// How much the viewing angle should influence the positioning of the eye
    #[serde(rename = "$parallaxstrength", default = "default_parallax_strength")]
    pub parallax_strength: f32,
    /// Dilates the pupil using the cornea texture to determine the shape of dilation. Default 0.5.
    #[serde(rename = "$dilation", default = "default_dilation")]
    pub dilation: f32,

    /// 1-dimensional texture which remaps lighting colors.
    #[serde(rename = "$lightwarptexture", deserialize_with = "deserialize_path")]
    pub light_warp_texture: String,
    /// Enables cubemap reflections. This shader has a specific cubemap made for it, engine/eye-reflection-cubemap-.vtf, but others can be used, including env_cubemap.
    #[serde(rename = "$envmap", deserialize_with = "deserialize_path")]
    pub env_map: String,
    /// The opacity of the cubemap reflection. Does not affect the eye glint. Default 0.5.
    #[serde(rename = "$glossiness", default = "default_dilation")]
    pub glossiness: f32,

    /// An ambient occlusion texture overlaid onto the entire eye
    #[serde(rename = "$ambientoccltexture", deserialize_with = "deserialize_path")]
    pub ambient_occlusion_texture: String,
    /// Tints the $ambientoccltexture
    #[serde(rename = "$ambientocclcolor", default = "default_occl_color")]
    pub ambient_occlusion_color: Vec3,
    /// Strength of the dynamic ambient occlusion.
    #[serde(rename = "$ambientocclusion", default = "default_scale")]
    pub ambiento_cclusion: f32,

    /// Enables half-lambertian lighting
    #[serde(rename = "$halflambert", default)]
    pub half_lambert: bool,
    /// Enables sphere raytracing. Each pixel is raytraced to allow sharper angles to look more accurate.
    #[serde(rename = "$raytracesphere", default)]
    pub ray_trace_sphere: bool,
    /// Requires $raytracesphere 1. Causes pixels which don't hit the raytraced sphere to be transparent, instead of rendering the "non-raytraced" eye behind it.
    #[serde(rename = "$spheretexkillcombo", default)]
    pub sphere_tex_kill_combo: bool,
    /// Requires $raytracesphere 1. Radius of the eyeball. Should be the diameter of the eyeball divided by 2
    #[serde(rename = "$eyeballradius", default = "default_dilation")]
    pub eye_ball_radius: f32,
}

fn default_parallax_strength() -> f32 {
    0.25
}

fn default_dilation() -> f32 {
    0.5
}

fn default_occl_color() -> Vec3 {
    Vec3([0.33; 3])
}
