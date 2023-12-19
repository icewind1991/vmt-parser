mod eyerefract;
mod lightmappedgeneric;
mod unlitgeneric;
mod unlittwotexture;
mod vertexlitgeneric;
mod water;
mod worldvertextransition;

pub use eyerefract::EyeRefractMaterial;
pub use lightmappedgeneric::LightMappedGenericMaterial;
use serde::{Deserialize, Deserializer, Serialize};
pub use unlitgeneric::UnlitGenericMaterial;
pub use unlittwotexture::UnlitTwoTextureMaterial;
use vdf_reader::entry::{Entry, Table};
use vdf_reader::error::UnknownError;
use vdf_reader::{from_entry, VdfError};
pub use vertexlitgeneric::VertexLitGenericMaterial;
pub use water::WaterMaterial;
pub use worldvertextransition::WorldVertexTransitionMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
    #[serde(rename = "lightmappedgeneric")]
    LightMappedGeneric(LightMappedGenericMaterial),
    #[serde(rename = "vertexlitgeneric")]
    VertexLitGeneric(VertexLitGenericMaterial),
    #[serde(rename = "unlitgeneric")]
    UnlitGeneric(UnlitGenericMaterial),
    #[serde(rename = "unlittwotexture")]
    UnlitTwoTexture(UnlitTwoTextureMaterial),
    #[serde(rename = "water")]
    Water(WaterMaterial),
    #[serde(rename = "worldvertextransition")]
    WorldVertexTransition(WorldVertexTransitionMaterial),
    #[serde(rename = "eyerefract")]
    EyeRefract(EyeRefractMaterial),
    #[serde(rename = "patch")]
    Patch(PatchMaterial),
}

impl Material {
    /// If the material is a patch, apply it to the included material
    pub fn resolve<E, Loader>(self, loader: Loader) -> Result<Material, E>
    where
        Loader: FnOnce(&str) -> Result<String, E>,
        E: From<VdfError>,
    {
        match self {
            Material::Patch(patch) => patch.resolve(loader),
            mat => Ok(mat),
        }
    }

    pub fn translucent(&self) -> bool {
        match self {
            Material::LightMappedGeneric(mat) => mat.translucent,
            Material::VertexLitGeneric(mat) => mat.translucent,
            Material::UnlitGeneric(mat) => mat.translucent,
            Material::UnlitTwoTexture(mat) => mat.translucent,
            Material::WorldVertexTransition(mat) => mat.translucent,
            Material::Water(_) => true,
            _ => false,
        }
    }

    pub fn no_cull(&self) -> bool {
        match self {
            Material::LightMappedGeneric(mat) => mat.no_cull,
            Material::VertexLitGeneric(mat) => mat.no_cull,
            Material::UnlitGeneric(mat) => mat.no_cull,
            Material::UnlitTwoTexture(mat) => mat.no_cull,
            Material::WorldVertexTransition(mat) => mat.no_cull,
            Material::Water(_) => true,
            _ => false,
        }
    }

    pub fn alpha_test(&self) -> Option<f32> {
        match self {
            Material::LightMappedGeneric(mat) => mat.alpha_test.then_some(mat.alpha_test_reference),
            Material::VertexLitGeneric(mat) => mat.alpha_test.then_some(mat.alpha_test_reference),
            Material::UnlitGeneric(mat) => mat.alpha_test.then_some(mat.alpha_test_reference),
            Material::UnlitTwoTexture(mat) => mat.alpha_test.then_some(mat.alpha_test_reference),
            Material::WorldVertexTransition(mat) => {
                mat.alpha_test.then_some(mat.alpha_test_reference)
            }
            Material::Water(_) => None,
            _ => None,
        }
    }

    pub fn base_texture(&self) -> &str {
        match self {
            Material::LightMappedGeneric(mat) => &mat.base_texture,
            Material::VertexLitGeneric(mat) => &mat.base_texture,
            Material::UnlitGeneric(mat) => &mat.base_texture,
            Material::UnlitTwoTexture(mat) => &mat.base_texture,
            Material::WorldVertexTransition(mat) => &mat.base_texture,
            Material::Water(mat) => mat.base_texture.as_deref().unwrap_or_default(),
            Material::EyeRefract(mat) => &mat.iris,
            _ => "",
        }
    }

    pub fn bump_map(&self) -> Option<&str> {
        match self {
            Material::LightMappedGeneric(mat) => mat.bump_map.as_deref(),
            Material::VertexLitGeneric(mat) => mat.bump_map.as_deref(),
            Material::UnlitGeneric(mat) => mat.bump_map.as_deref(),
            Material::UnlitTwoTexture(mat) => mat.bump_map.as_deref(),
            Material::WorldVertexTransition(mat) => mat.bump_map.as_deref(),
            Material::Water(mat) => mat.bump_map.as_deref(),
            _ => None,
        }
    }

    pub fn surface_prop(&self) -> Option<&str> {
        match self {
            Material::LightMappedGeneric(mat) => mat.surface_prop.as_deref(),
            Material::UnlitGeneric(mat) => mat.surface_prop.as_deref(),
            Material::UnlitTwoTexture(mat) => mat.surface_prop.as_deref(),
            Material::WorldVertexTransition(mat) => mat.surface_prop.as_deref(),
            _ => None,
        }
    }

    pub fn alpha(&self) -> f32 {
        match self {
            Material::LightMappedGeneric(mat) => mat.alpha,
            Material::VertexLitGeneric(mat) => mat.alpha,
            Material::UnlitGeneric(mat) => mat.alpha,
            Material::UnlitTwoTexture(mat) => mat.alpha,
            Material::WorldVertexTransition(mat) => mat.alpha,
            _ => 1.0,
        }
    }

    pub fn ignore_z_test(&self) -> bool {
        match self {
            Material::LightMappedGeneric(mat) => mat.ignore_z,
            Material::VertexLitGeneric(mat) => mat.ignore_z,
            Material::UnlitGeneric(mat) => mat.ignore_z,
            Material::UnlitTwoTexture(mat) => mat.ignore_z,
            Material::WorldVertexTransition(mat) => mat.ignore_z,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMaterial {
    #[serde(deserialize_with = "deserialize_path")]
    include: String,
    #[serde(default)]
    replace: Table,
}

impl PatchMaterial {
    /// Load the included material and apply the patch
    pub fn resolve<E, Loader>(&self, loader: Loader) -> Result<Material, E>
    where
        Loader: FnOnce(&str) -> Result<String, E>,
        E: From<VdfError>,
    {
        let base = loader(&self.include)?.to_ascii_lowercase();
        let mut material = Table::load_from_str(&base)?;

        let material_values = match material.iter_mut().next() {
            Some((_, Entry::Table(table))) => table,
            _ => {
                return Err(VdfError::from(UnknownError::from(
                    "included vdf doesn't look like a material",
                ))
                .into())
            }
        };
        for (key, value) in self.replace.iter() {
            material_values.insert(key.clone(), value.clone());
        }

        Ok(from_entry(Entry::Table(material))?)
    }
}

trait PathLike {
    fn normalize(self) -> Self;
}

impl PathLike for String {
    fn normalize(self) -> Self {
        self.replace('\\', "/")
    }
}

impl<T: PathLike> PathLike for Option<T> {
    fn normalize(self) -> Self {
        self.map(T::normalize)
    }
}

/// Deserialize string and convert windows \ path separators to /
fn deserialize_path<'de, T: PathLike + Deserialize<'de>, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<T, D::Error> {
    Ok(T::deserialize(deserializer)?.normalize())
}
