use super::deserialize_path;
use crate::{
    default_detail_scale, default_scale, default_scale3, BlendMode, TextureTransform, Vec2, Vec3,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefractMaterial {
    /// The pattern of refraction is defined by a normal map (DX9+) or DUDV map (DX8-). May be animated.
    #[serde(rename = "$normalmap", deserialize_with = "deserialize_path")]
    pub normal_map: String,
    /// The pattern of refraction is defined by a normal map (DX9+) or DUDV map (DX8-). May be animated.
    #[serde(rename = "$dudvmap", default, deserialize_with = "deserialize_path")]
    pub du_dv_map: Option<String>,
    /// If a second normal map is specified, it will be blended with the first one.
    #[serde(rename = "$normalmap2", default, deserialize_with = "deserialize_path")]
    pub normal_map2: Option<String>,
    /// Use a texture instead of rendering the view for the source of the distorted pixels.
    #[serde(
        rename = "$basetexture",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub base_texture: Option<String>,
    /// Transforms the bump map texture.
    #[serde(rename = "$bumptransform", default)]
    pub bump_transform: TextureTransform,
    /// Transforms the bump map texture.
    #[serde(rename = "$bumptransform2", default)]
    pub bump_transform2: TextureTransform,

    #[serde(rename = "$refracttint", default = "default_scale3")]
    pub refract_tint: Vec3,
    /// Tints the colour of the refraction either uniformly or per-texel. Can be used in conjunction with $refracttint
    #[serde(
        rename = "$refracttinttexture",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub refract_tint_texture: Option<String>,
    /// Controls the strength of the refraction by multiplying the normal map intensity.
    #[serde(rename = "$refractamount", default = "default_scale")]
    pub refract_amount: f32,
    /// Adds a blur effect. Valid values are 0, 1 and 2 (0 and 1 for DX8-).
    #[serde(rename = "$bluramount", default)]
    pub blur_amount: f32,

    #[serde(rename = "$decalscale", default = "default_detail_scale")]
    /// Fits the detail texture onto the material the given number of times
    pub detail_scale: Vec2,
    /// Controls the amount that the detail texture affects the base texture. The precise use of this depends on the blend factor; in most cases it acts similarly to $alpha. A value of 0 usually makes the detail texture have no effect, whilst a value of 1 applies the full effect.
    #[serde(rename = "$detailblendfactor", default = "default_scale")]
    pub detail_blend_factor: f32,
    /// How to combine the detail material with the albedo.
    #[serde(rename = "$detailblendmode", default)]
    pub detail_blend_mode: BlendMode,
    /// A separate VertexLitGeneric material to that will replace this one if the decal hits a model.
    #[serde(
        rename = "$modelmaterial",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub model_material: Option<String>,
    /// Disables texture filtering.
    #[serde(rename = "$pointsamplemagfilter", default)]
    pub point_sample_mag_filter: bool,
    /// Mitigation for displacement texture stretching.
    #[serde(rename = "$seamless_scale", default = "default_scale")]
    pub seamless_scale: f32,

    /// Scales the opacity of an entire material.
    #[serde(rename = "$alpha", default = "default_scale")]
    pub alpha: f32,
    /// Specifies a mask to use to determine binary opacity.
    #[serde(rename = "$alphatest", default)]
    pub alpha_test: bool,
    /// Specifies a mask to use to determine binary opacity.
    #[serde(rename = "$alphatestreference", default = "default_scale")]
    pub alpha_test_reference: f32,
    /// Vector-like edge filtering.
    #[serde(rename = "$distancealpha", default)]
    pub distance_alpha: bool,
    /// Disables backface culling.
    #[serde(rename = "$nocull", default)]
    pub no_cull: bool,
    /// Specifies that the material should be partially see-through.
    #[serde(rename = "$translucent", default)]
    pub translucent: bool,

    /// Specifies a texture that will provide three-dimensional lighting information for a material.
    #[serde(rename = "$bumpmap", default, deserialize_with = "deserialize_path")]
    pub bump_map: Option<String>,
    /// Per-texel color modification via a warp texture.
    #[serde(
        rename = "$lightwarptexture",
        default,
        deserialize_with = "deserialize_path"
    )]
    pub light_wrap_texture: Option<String>,
    /// Determines whether the surface is self-illuminated independent of environment lighting.
    #[serde(rename = "$selfillum", default)]
    pub self_illum: bool,
    /// Flags the $bumpmap as being a self-shadowing bumpmap.
    #[serde(rename = "$ssbump", default)]
    pub ss_bump: bool,

    /// Specular reflections.
    #[serde(rename = "$envmap", default, deserialize_with = "deserialize_path")]
    pub env_map: Option<String>,
    #[serde(rename = "$envmaptint", default = "default_scale3")]
    pub env_map_tint: Vec3,
}
