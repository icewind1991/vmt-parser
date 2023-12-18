use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct TextureTransform {
    pub center: [f32; 2],
    pub scale: [f32; 2],
    pub rotate: f32,
    pub translate: [f32; 2],
}

impl Default for TextureTransform {
    fn default() -> Self {
        TextureTransform {
            center: [0.5; 2],
            scale: [1.0; 2],
            rotate: 0.0,
            translate: [0.0; 2],
        }
    }
}

impl Display for TextureTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "center {} {} scale {} {} rotate {} translate {} {}",
            self.center[0],
            self.center[1],
            self.scale[0],
            self.scale[1],
            self.rotate,
            self.translate[0],
            self.translate[1],
        )
    }
}

#[test]
fn test_parse_base_texture_transform() {
    assert_eq!(
        TextureTransform {
            center: [0.5; 2],
            scale: [1.0; 2],
            rotate: 0.0,
            translate: [0.0; 2],
        },
        TextureTransform::from_str("center .5 .5 scale 1 1 rotate 0 translate 0 0").unwrap()
    );
    assert_eq!(
        TextureTransform {
            center: [0.2, 0.3],
            scale: [1.1, 1.2],
            rotate: 1.0,
            translate: [0.4, 0.5],
        },
        TextureTransform::from_str("center .2 .3 scale 1.1 1.2 rotate 1 translate 0.4 0.5")
            .unwrap()
    );
}

impl FromStr for TextureTransform {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parts = str.split(' ').filter(|p| !p.is_empty());
        match (
            parts.next(),
            parts.next().and_then(|val| f32::from_str(val).ok()),
            parts.next().and_then(|val| f32::from_str(val).ok()),
            parts.next(),
            parts.next().and_then(|val| f32::from_str(val).ok()),
            parts.next().and_then(|val| f32::from_str(val).ok()),
            parts.next(),
            parts.next().and_then(|val| f32::from_str(val).ok()),
            parts.next(),
            parts.next().and_then(|val| f32::from_str(val).ok()),
            parts.next().and_then(|val| f32::from_str(val).ok()),
        ) {
            (
                Some("center"),
                Some(center_x),
                Some(center_y),
                Some("scale"),
                Some(scale_x),
                Some(scale_y),
                Some("rotate"),
                Some(rotate),
                Some("translate"),
                Some(translate_x),
                Some(translate_y),
            ) => Ok(TextureTransform {
                center: [center_x, center_y],
                scale: [scale_x, scale_y],
                rotate,
                translate: [translate_x, translate_y],
            }),
            _ => Err("invalid $basetexturetransform format"),
        }
    }
}

impl<'de> Deserialize<'de> for TextureTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = Cow::<str>::deserialize(deserializer)?;
        Self::from_str(str.as_ref()).map_err(D::Error::custom)
    }
}

impl Serialize for TextureTransform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
