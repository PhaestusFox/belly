use bevy::{
    prelude::{Color, Deref},
    ui::{UiRect, Val},
    utils::HashMap,
};
use cssparser::{BasicParseErrorKind, Token};
use smallvec::SmallVec;
use tagstr::Tag;

use crate::ElementsError;

use super::{colors, PropertyValue};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub struct Number([u8; 4]);

impl Number {
    fn from_float(value: f32) -> Self {
        Number(value.to_le_bytes())
    }
    fn to_float(&self) -> f32 {
        f32::from_le_bytes(self.0)
    }
}

impl From<f32> for Number {
    fn from(v: f32) -> Self {
        Number::from_float(v)
    }
}
impl From<&f32> for Number {
    fn from(v: &f32) -> Self {
        Number::from_float(*v)
    }
}

impl From<Number> for f32 {
    fn from(v: Number) -> Self {
        v.to_float()
    }
}

impl From<&Number> for f32 {
    fn from(v: &Number) -> Self {
        v.to_float()
    }
}

/// A property value token which was parsed from a CSS rule.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum StylePropertyToken {
    /// A value which was parsed percent value, like `100%` or `73.23%`.
    Percentage(Number),
    /// A value which was parsed dimension value, like `10px` or `35em.
    ///
    /// Currently there is no distinction between [`length-values`](https://developer.mozilla.org/en-US/docs/Web/CSS/length).
    Dimension(Number),
    /// A numeric float value, like `31.1` or `43`.
    Number(Number),
    /// A plain identifier, like `none` or `center`.
    Identifier(String),
    /// A identifier prefixed by a hash, like `#001122`.
    Hash(String),
    /// A quoted string, like `"some value"`.
    String(String),
    /// Property delimiter (comma or slash)
    Slash,
    Comma,
}

impl StylePropertyToken {
    fn to_string(&self) -> String {
        match self {
            StylePropertyToken::Percentage(v) => format!("{}%", v.to_float()),
            StylePropertyToken::Dimension(v) => format!("{}px", v.to_float()),
            StylePropertyToken::Number(v) => format!("{}", v.to_float()),
            StylePropertyToken::Identifier(v) => format!("{}", v),
            StylePropertyToken::Hash(v) => format!("#{}", v),
            StylePropertyToken::String(v) => format!("\"{}\"", v),
            StylePropertyToken::Slash => format!("/"),
            StylePropertyToken::Comma => format!(","),
        }
    }

    fn val(&self) -> Result<Val, ElementsError> {
        match self {
            StylePropertyToken::Percentage(p) => Ok(Val::Percent(p.to_float())),
            StylePropertyToken::Dimension(d) => Ok(Val::Px(d.to_float())),
            StylePropertyToken::Identifier(i) if i == "auto" => Ok(Val::Auto),
            StylePropertyToken::Identifier(i) if i == "undefined" => Ok(Val::Undefined),
            _ => Err(ElementsError::InvalidPropertyValue(format!(
                "Can't treat `{}` as size value",
                self.to_string()
            ))),
        }
    }

    fn is_delimiter(&self) -> bool {
        match self {
            Self::Slash | Self::Comma => true,
            _ => false,
        }
    }
}

impl<'i> TryFrom<Token<'i>> for StylePropertyToken {
    type Error = String;

    fn try_from(token: Token<'i>) -> Result<Self, Self::Error> {
        match token {
            Token::Ident(val) => Ok(Self::Identifier(val.to_string())),
            Token::Hash(val) => Ok(Self::Hash(val.to_string())),
            Token::IDHash(val) => Ok(Self::Hash(val.to_string())),
            Token::QuotedString(val) => Ok(Self::String(val.to_string())),
            Token::Number { value, .. } => Ok(Self::Number(value.into())),
            Token::Percentage { unit_value, .. } => {
                Ok(Self::Percentage((unit_value * 100.0).into()))
            }
            Token::Dimension { value, .. } => Ok(Self::Dimension(value.into())),
            Token::Comma => Ok(Self::Comma),
            Token::Delim(d) if d == '/' => Ok(Self::Slash),
            token => Err(format!("Invalid token: {:?}", token)),
        }
    }
}

/// A list of [`PropertyToken`] which was parsed from a single property.
#[derive(Debug, Default, Clone, Deref, PartialEq, Eq, Hash)]
pub struct StyleProperty(pub(crate) SmallVec<[StylePropertyToken; 8]>);

impl StyleProperty {
    pub fn as_stream(&self) -> StylePropertyTokenStream {
        StylePropertyTokenStream {
            offset: 0,
            tokens: self,
        }
    }
}

pub struct StylePropertyTokenStream<'a> {
    offset: usize,
    tokens: &'a StyleProperty,
}

impl<'a> StylePropertyTokenStream<'a> {
    pub fn single(&mut self) -> Option<&[StylePropertyToken]> {
        if self.offset >= self.tokens.len() {
            None
        } else {
            let start = self.offset;
            let end = self.offset + 1;
            self.offset += 1;
            if self.offset < self.tokens.len() && self.tokens[self.offset].is_delimiter() {
                self.offset += 1;
            }
            Some(&self.tokens[start..end])
        }
    }
    pub fn compound(&mut self) -> Option<&[StylePropertyToken]> {
        if self.offset >= self.tokens.len() {
            return None;
        }
        let start = self.offset;
        let mut end = self.offset;
        while self.offset < self.tokens.len() {
            self.offset += 1;
            end = self.offset;
            if self.offset < self.tokens.len() && self.tokens[self.offset].is_delimiter() {
                self.offset += 1;
                break;
            }
        }
        Some(&self.tokens[start..end])
    }
}

impl From<&StyleProperty> for StyleProperty {
    fn from(v: &StyleProperty) -> Self {
        v.clone()
    }
}

pub trait StylePropertyMethods {
    fn tokens(&self) -> &[StylePropertyToken];
    fn hello(&self) {}
    fn to_string(&self) -> String {
        let mut result = "".to_string();
        for value in self.tokens().iter() {
            result.push_str(&value.to_string());
        }
        result
    }
    /// Tries to parses the current values as a single [`String`].
    fn string(&self) -> Result<String, ElementsError> {
        let Some(token) = self.tokens().iter().next() else {
            return Err(ElementsError::InvalidPropertyValue(format!("Expected string literal, got nothing")));
        };
        match token {
            StylePropertyToken::String(id) => Ok(id.clone()),
            e => Err(ElementsError::InvalidPropertyValue(format!(
                "Expected string literal, got {}",
                e.to_string()
            ))),
        }
    }

    /// Tries to parses the current values as a single [`Option<UiRect>`].
    ///
    /// Optional values are handled by this function, so if only one value is present it is used as `top`, `right`, `bottom` and `left`,
    /// otherwise values are applied in the following order: `top`, `right`, `bottom` and `left`.
    ///
    /// Note that it is not possible to create a [`UiRect`] with only `top` value, since it'll be understood to replicated it on all fields.
    fn rect(&self) -> Result<UiRect, ElementsError> {
        let props = self.tokens();
        match props.len() {
            1 => props[0].val().map(UiRect::all),
            2 => {
                let top_bottom = props[0].val()?;
                let left_right = props[1].val()?;
                Ok(UiRect::new(left_right, left_right, top_bottom, top_bottom))
            }
            3 => {
                let top = props[0].val()?;
                let left_right = props[1].val()?;
                let bottom = props[2].val()?;
                Ok(UiRect::new(left_right, left_right, top, bottom))
            }
            4 => {
                let top = props[0].val()?;
                let right = props[1].val()?;
                let bottom = props[2].val()?;
                let left = props[3].val()?;
                Ok(UiRect::new(left, right, top, bottom))
            }
            _ => Err(ElementsError::InvalidPropertyValue(format!(
                "Can't extract rect from `{}`",
                props.to_string()
            ))),
        }
    }

    fn rect_map(&self, prefix: &str) -> Result<HashMap<Tag, PropertyValue>, ElementsError> {
        let rect = self.tokens().rect()?;
        Ok(rect.to_rect_map(prefix))
    }

    /// Tries to parses the current values as a single [`Color`].
    ///
    /// Currently only [named colors](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color)
    /// and [hex-colors](https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color) are supported.
    fn color(&self) -> Result<Color, ElementsError> {
        let props = self.tokens();
        if props.len() == 0 {
            return Err(ElementsError::InvalidPropertyValue(format!(
                "Expected color, got nothing"
            )));
        }
        match &props[0] {
            StylePropertyToken::Identifier(name) => colors::parse_named_color(name.as_str())
                .ok_or_else(|| {
                    ElementsError::InvalidPropertyValue(format!("Unknown color name '{name}'"))
                }),
            StylePropertyToken::Hash(hash) => colors::parse_hex_color(hash.as_str()),
            prop => {
                return Err(ElementsError::InvalidPropertyValue(format!(
                    "Can't parse color from {}",
                    prop.to_string()
                )))
            }
        }
    }

    /// Tries to parses the current values as a single identifier.
    fn identifier(&self) -> Option<&str> {
        self.tokens().iter().find_map(|token| match token {
            StylePropertyToken::Identifier(id) => {
                if id.is_empty() {
                    None
                } else {
                    Some(id.as_str())
                }
            }
            _ => None,
        })
    }

    /// Tries to parses the current values as a single [`Val`].
    ///
    /// Only [`Percentage`](PropertyToken::Percentage) and [`Dimension`](PropertyToken::Dimension`) are considered valid values,
    /// where former is converted to [`Val::Percent`] and latter is converted to [`Val::Px`].
    fn val(&self) -> Result<Val, ElementsError> {
        let Some(prop) = self.tokens().iter().next() else {
            return Err(ElementsError::InvalidPropertyValue(format!("Expected Val, found none")))
        };
        match prop {
            StylePropertyToken::Percentage(val) => Ok(Val::Percent(val.into())),
            StylePropertyToken::Dimension(val) => Ok(Val::Px(val.into())),
            StylePropertyToken::Identifier(val) if val.as_str() == "auto" => Ok(Val::Auto),
            StylePropertyToken::Identifier(val) if val.as_str() == "undefined" => {
                Ok(Val::Undefined)
            }
            p => Err(ElementsError::InvalidPropertyValue(format!(
                "Can't parrse Val from '{}'",
                p.to_string()
            ))),
        }
    }

    /// Tries to parses the current values as a single [`f32`].
    ///
    /// Only [`Percentage`](PropertyToken::Percentage), [`Dimension`](PropertyToken::Dimension`) and [`Number`](PropertyToken::Number`)
    /// are considered valid values.
    fn f32(&self) -> Result<f32, ElementsError> {
        let Some(prop) = self.tokens().iter().next() else {
            return Err(ElementsError::InvalidPropertyValue(format!("Expected f32, found none")))
        };
        match prop {
            StylePropertyToken::Percentage(val)
            | StylePropertyToken::Dimension(val)
            | StylePropertyToken::Number(val) => Ok(val.into()),
            p => Err(ElementsError::InvalidPropertyValue(format!(
                "Can't parse f32 from '{}'",
                p.to_string()
            ))),
        }
    }

    /// Tries to parses the current values as a single [`Option<f32>`].
    ///
    /// This function is useful for properties where either a numeric value or a `none` value is expected.
    ///
    /// If a [`Option::None`] is returned, it means some invalid value was found.
    ///
    /// If there is a [`Percentage`](PropertyToken::Percentage), [`Dimension`](PropertyToken::Dimension`) or [`Number`](PropertyToken::Number`) token,
    /// a [`Option::Some`] with parsed [`Option<f32>`] is returned.
    /// If there is a identifier with a `none` value, then [`Option::Some`] with [`None`] is returned.
    fn option_f32(&self) -> Result<Option<f32>, ElementsError> {
        let Some(prop) = self.tokens().iter().next() else {
            return Err(ElementsError::InvalidPropertyValue(format!("Expected Option<f32>, found none")))
        };
        match prop {
            StylePropertyToken::Percentage(val)
            | StylePropertyToken::Dimension(val)
            | StylePropertyToken::Number(val) => Ok(Some(val.into())),
            StylePropertyToken::Identifier(ident) => match ident.as_str() {
                "none" => Ok(None),
                ident => Err(ElementsError::InvalidPropertyValue(format!(
                    "Can't parse Option<f32> from {ident}"
                ))),
            },
            e => Err(ElementsError::InvalidPropertyValue(format!(
                "Can't parse Option<f32> from {}",
                e.to_string()
            ))),
        }
    }
}
impl StylePropertyMethods for &[StylePropertyToken] {
    fn tokens(&self) -> &[StylePropertyToken] {
        self
    }
}

impl StylePropertyMethods for StyleProperty {
    fn tokens(&self) -> &[StylePropertyToken] {
        &self.0[..]
    }
}

pub trait ToRectMap {
    fn to_rect_map(&self, prefix: &str) -> HashMap<Tag, PropertyValue>;
}

impl ToRectMap for UiRect {
    fn to_rect_map(&self, prefix: &str) -> HashMap<Tag, PropertyValue> {
        let mut props = HashMap::default();
        let prefix = prefix.to_string();
        props.insert(
            Tag::new(prefix.clone() + "-left"),
            PropertyValue::new(self.left),
        );
        props.insert(
            Tag::new(prefix.clone() + "-right"),
            PropertyValue::new(self.right),
        );
        props.insert(
            Tag::new(prefix.clone() + "-top"),
            PropertyValue::new(self.top),
        );
        props.insert(
            Tag::new(prefix.clone() + "-bottom"),
            PropertyValue::new(self.bottom),
        );
        props
    }
}

fn parse_style_propery_value(value: &str) -> Result<StyleProperty, ElementsError> {
    let mut input = cssparser::ParserInput::new(value);
    let mut parser = cssparser::Parser::new(&mut input);
    let mut values: SmallVec<[StylePropertyToken; 8]> = SmallVec::new();
    loop {
        let next = parser.next();
        match next {
            Ok(token) => values.push(token.clone().try_into().map_err(|e| {
                ElementsError::InvalidPropertyValue(format!(
                    "Can't parse `{}` (invalid token `{:?}`: {:?}",
                    value, token, e
                ))
            })?),
            Err(e) if e.kind == BasicParseErrorKind::EndOfInput => break,
            Err(e) => {
                return Err(ElementsError::InvalidPropertyValue(format!(
                    "Can't parse `{}`: {:?}",
                    value, e
                )))
            }
        }
    }
    Ok(StyleProperty(values))
}

impl TryFrom<&str> for StyleProperty {
    type Error = ElementsError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_style_propery_value(value)
    }
}

impl TryFrom<String> for StyleProperty {
    type Error = ElementsError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        parse_style_propery_value(&value)
    }
}

impl TryFrom<&String> for StyleProperty {
    type Error = ElementsError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        parse_style_propery_value(value.as_str())
    }
}