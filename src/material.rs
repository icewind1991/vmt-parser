use crate::error::{EofError, ParseValueError};
use crate::Result;
use std::collections::HashMap;
use vdf_reader::entry::{Entry, FromEntry, Table};
use vdf_reader::{Event, GroupStartEvent, Reader, VdfError};

#[derive(Debug, Clone)]
pub enum Material {
    Other(OtherMaterial),
    LightMappedGeneric(LightMappedGenericMaterial),
}

impl Material {
    pub fn parse(reader: &mut Reader) -> Result<Self> {
        let start: GroupStartEvent = expect_event(reader, "material name")?;
        let name = start.name.to_ascii_lowercase();

        Ok(match name.as_str() {
            "lightmappedgeneric" => {
                Material::LightMappedGeneric(LightMappedGenericMaterial::parse(reader)?)
            }
            _ => Material::Other(OtherMaterial::parse(name, reader)?),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LightMappedGenericMaterial {
    pub keywords: String,
    pub detail: Option<String>,
    pub detail_blend_factory: f32,
    pub detail_scale: f32,
    pub detail_blend_mode: u32,
    pub base_texture: String,
    pub ss_bump: bool,
    pub bump_map: Option<String>,
    pub rest: HashMap<String, Entry>,
}

impl LightMappedGenericMaterial {
    fn parse(reader: &mut Reader) -> Result<Self> {
        let src = reader.source;
        let mut keywords = Default::default();
        let mut detail = Default::default();
        let mut detail_blend_factory = Default::default();
        let mut detail_scale = Default::default();
        let mut detail_blend_mode = Default::default();
        let mut base_texture = Default::default();
        let mut ss_bump = Default::default();
        let mut bump_map = Default::default();

        let mut rest: HashMap<String, Entry> = Default::default();

        loop {
            let (span, key, value) = match event(reader, "item or group end")? {
                Event::GroupEnd(_) => break,
                Event::GroupStart(start) => (
                    start.span,
                    start.name.to_ascii_lowercase(),
                    Entry::Table(Table::load(reader)?),
                ),
                Event::Entry(entry) => (
                    entry.span,
                    entry.key.into_content().to_ascii_lowercase(),
                    entry.value.into(),
                ),
            };
            match key.as_str() {
                "%keywords" => {
                    keywords = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "keywords", span.into(), err)
                    })?;
                }
                "$detail" => {
                    detail = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "detail", span.into(), err)
                    })?;
                }
                "$detailblendfactor" => {
                    detail_blend_factory = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "detail_blend_factory", span.into(), err)
                    })?;
                }
                "$detailscale" => {
                    detail_scale = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "detail_scale", span.into(), err)
                    })?;
                }
                "$detailblendmode" => {
                    detail_blend_mode = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "detail_blend_mode", span.into(), err)
                    })?;
                }
                "$basetexture" => {
                    base_texture = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "base_texture", span.into(), err)
                    })?;
                }
                "$ssbump" => {
                    ss_bump = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "ss_bump", span.into(), err)
                    })?;
                }
                "$bumpmap" => {
                    bump_map = FromEntry::from_entry(value).map_err(|err| {
                        ParseValueError::new(src.into(), "bump_map", span.into(), err)
                    })?;
                }
                _ => {
                    rest.insert(key, value);
                }
            }
        }

        Ok(LightMappedGenericMaterial {
            keywords,
            detail,
            detail_blend_factory,
            detail_scale,
            detail_blend_mode,
            base_texture,
            ss_bump,
            bump_map,
            rest,
        })
    }
}

#[derive(Debug, Clone)]
pub struct OtherMaterial {
    pub name: String,
    pub values: HashMap<String, Entry>,
}

impl OtherMaterial {
    fn parse(name: String, reader: &mut Reader) -> Result<Self> {
        let mut values: HashMap<String, _> = HashMap::new();

        loop {
            let (key, value) = match event(reader, "item or group end")? {
                Event::GroupEnd(_) => break,
                Event::GroupStart(start) => (
                    start.name.to_ascii_lowercase(),
                    Entry::Table(Table::load(reader)?),
                ),
                Event::Entry(entry) => (
                    entry.key.into_content().to_ascii_lowercase(),
                    entry.value.into(),
                ),
            };
            values.insert(key, value);
        }

        Ok(OtherMaterial { name, values })
    }
}

fn event<'a>(reader: &mut Reader<'a>, expected: &'static str) -> Result<Event<'a>> {
    Ok(reader
        .next()
        .ok_or_else(|| EofError::new(reader.source.into(), expected))??)
}

fn expect_event<'a, E: 'a>(reader: &mut Reader<'a>, expected: &'static str) -> Result<E>
where
    E: TryFrom<Event<'a>, Error = VdfError>,
{
    let event = event(reader, expected)?;
    E::try_from(event).map_err(|e| {
        match e {
            VdfError::WrongEntryType(e) => e.with_source(reader.source.into()).into(),
            e => e,
        }
        .into()
    })
}
