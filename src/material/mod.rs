mod lightmappedgeneric;
mod unlitgeneric;
mod water;
mod worldvertextransition;

pub use lightmappedgeneric::LightMappedGenericMaterial;
use serde::{Deserialize, Serialize};
pub use unlitgeneric::UnlitGenericMaterial;
use vdf_reader::entry::{Entry, Table};
use vdf_reader::error::UnknownError;
use vdf_reader::{from_entry, VdfError};
pub use water::WaterMaterial;
pub use worldvertextransition::WorldVertexTransitionMaterial;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
    #[serde(rename = "lightmappedgeneric")]
    LightMappedGeneric(LightMappedGenericMaterial),
    #[serde(rename = "unlitgeneric")]
    UnlitGeneric(UnlitGenericMaterial),
    #[serde(rename = "water")]
    Water(WaterMaterial),
    #[serde(rename = "worldvertextransition")]
    WorldVertexTransition(WorldVertexTransitionMaterial),
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMaterial {
    include: String,
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
