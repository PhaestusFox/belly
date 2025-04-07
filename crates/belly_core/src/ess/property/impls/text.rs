use super::parse;
use crate::ess::defaults::Defaults;
use crate::ess::PropertyParser;
use crate::ess::StyleProperty;
use crate::ess::StylePropertyToken;
use crate::style_property;
use crate::ElementsError;
use bevy::prelude::*;

#[derive(Default, Clone)]
pub enum FontPath {
    #[default]
    Regular,
    Bold,
    Italic,
    BoldItalic,
    Custom(String),
}

/// regular|bold|italic|bold-italic|$string
pub struct FontParser;
impl PropertyParser<FontPath> for FontParser {
    fn parse(prop: &StyleProperty) -> Result<FontPath, ElementsError> {
        let Some(token) = prop.first() else {
            return Err(ElementsError::InvalidPropertyValue(
                "Expected regular|bold|italic|bold-italic|$string, got nothing".to_string(),
            ));
        };
        match token {
            StylePropertyToken::String(id) => Ok(FontPath::Custom(id.clone())),
            StylePropertyToken::Identifier(ident) => match ident.as_str() {
                "regular" => Ok(FontPath::Regular),
                "bold" => Ok(FontPath::Bold),
                "italic" => Ok(FontPath::Italic),
                "bold-italic" => Ok(FontPath::BoldItalic),
                ident => {
                    return Err(ElementsError::InvalidPropertyValue(format!(
                        "Expected regular|bold|italic|bold-italic|$string, got `{ident}`"
                    )))
                }
            },
            ident => Err(ElementsError::InvalidPropertyValue(format!(
                "Expected regulart|bold|italic|bold-italic|$string, got `{}`",
                ident.to_string()
            ))),
        }
    }
}

style_property! {
    #[doc = " TODO: wtite FontProperty description"]
    #[doc = " <!-- @property-category=Text -->"]
    FontProperty("font") {
        Default = "regular";
        Item = FontPath;
        Components = &'static mut TextFont;
        Filters = With<Node>;
        AffectsVirtual = true;
        Parser = FontParser;
        Apply = |value, text_font, assets, commands, entity| {
            if let FontPath::Custom(path) = value {
                text_font.font = assets.load(path);
            } else {
                let path = value.clone();
                commands.queue(move |world: &mut World| {
                    let defaults = world.resource::<Defaults>();
                    let font = match path {
                        FontPath::Regular => defaults.regular_font.clone(),
                        FontPath::Italic => defaults.italic_font.clone(),
                        FontPath::Bold => defaults.bold_font.clone(),
                        FontPath::BoldItalic => defaults.bold_italic_font.clone(),
                        _ => defaults.regular_font.clone(),
                    };
                    world
                        .entity_mut(entity)
                        .get_mut::<TextFont>()
                        .unwrap()
                        .font = font;
                });
            }
        };
    }
}

style_property! {
    #[doc = " TODO: remove depricate ColorProperty"]
    #[doc = " <!-- @property-category=Text -->"]
    ColorProperty("color") {
        Default = "#cfcfcf";
        Item = Color;
        Components = &'static mut TextColor;
        Filters = With<Node>;
        AffectsVirtual = true;
        Parser = parse::ColorParser;
        Apply = |value, text_color, _assets, _commands, _entity| {
            text_color.0 = *value;
        };
    }
}

style_property! {
    #[doc = " TODO: write FontSizeProperty description"]
    #[doc = " <!-- @property-category=Text -->"]
    FontSizeProperty("font-size") {
        Default = "24";
        Item = f32;
        Components = &'static mut TextFont;
        Filters = With<Node>;
        AffectsVirtual = true;
        Parser = parse::NumParser;
        Apply = |value, text_font, _assets, _commands, _entity| {
            text_font.font_size = *value;
        };
    }
}
