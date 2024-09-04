use bevy::{color::Alpha, prelude::Color};

use crate::ElementsError;

pub trait ColorFromHexExtension {
    fn from_hex<T: AsRef<str>>(color: T) -> Color {
        let color = color.as_ref().trim().trim_start_matches('#');
        parse_hex_color(color).unwrap_or_else(|_| Color::WHITE)
    }
    fn try_from_hex<T: AsRef<str>>(color: T) -> Result<Color, String> {
        let color = color.as_ref().trim().trim_start_matches('#');
        parse_hex_color(color).map_err(|e| format!("{e}"))
    }
    fn get_hex(&self) -> String;
    fn set_hex(&mut self, hex: impl AsRef<str>);
}
impl ColorFromHexExtension for Color {
    fn get_hex(&self) -> String {
        let c = self.to_srgba();
        let r = (c.red * 256.0) as u8;
        let g = (c.green * 256.0) as u8;
        let b = (c.blue * 256.0) as u8;
        let a = (c.alpha() * 256.0) as u8;
        let out = if a == 255 {
            format!("#{r:02x}{g:02x}{b:02x}")
        } else {
            format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
        };
        println!("hex = {}", out);
        out
    }
    fn set_hex(&mut self, hex: impl AsRef<str>) {
        *self = Self::from_hex(hex);
    }
}

pub(super) fn parse_hex_color(hex: &str) -> Result<Color, ElementsError> {
    let color = cssparser::Color::parse_hash(hex.as_bytes()).map_err(|_| {
        ElementsError::InvalidPropertyValue(format!("Can't parse color from '{hex}'"))
    })?;
    if let cssparser::Color::RGBA(cssparser::RGBA {
        red,
        green,
        blue,
        alpha,
    }) = color
    {
        Ok(Color::srgba(
            red as f32 / 255.,
            green as f32 / 255.,
            blue as f32 / 255.,
            alpha as f32 / 255.,
        ))
    } else {
        Err(ElementsError::InvalidPropertyValue(format!(
            "Can't parse color from '{hex}'"
        )))
    }
}

// Source: https://developer.mozilla.org/en-US/docs/Web/CSS/named-color

/// Parses a named color, like "silver" or "azure" into a [`Color`]
///
/// Accepts any [valid CSS named-colors](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color).
pub(super) fn parse_named_color(name: &str) -> Option<Color> {
    match name {
        // CSS Level 1 values
        "black" => Some(Color::srgba(0.0000, 0.0000, 0.0000, 1.0000)),
        "silver" => Some(Color::srgba(0.7529, 0.7529, 0.7529, 1.0000)),
        "gray" => Some(Color::srgba(0.5020, 0.5020, 0.5020, 1.0000)),
        "white" => Some(Color::srgba(1.0000, 1.0000, 1.0000, 1.0000)),
        "maroon" => Some(Color::srgba(0.5020, 0.0000, 0.0000, 1.0000)),
        "red" => Some(Color::srgba(1.0000, 0.0000, 0.0000, 1.0000)),
        "purple" => Some(Color::srgba(0.5020, 0.0000, 0.5020, 1.0000)),
        "fuchsia" => Some(Color::srgba(1.0000, 0.0000, 1.0000, 1.0000)),
        "green" => Some(Color::srgba(0.0000, 0.5020, 0.0000, 1.0000)),
        "lime" => Some(Color::srgba(0.0000, 1.0000, 0.0000, 1.0000)),
        "olive" => Some(Color::srgba(0.5020, 0.5020, 0.0000, 1.0000)),
        "yellow" => Some(Color::srgba(1.0000, 1.0000, 0.0000, 1.0000)),
        "navy" => Some(Color::srgba(0.0000, 0.0000, 0.5020, 1.0000)),
        "blue" => Some(Color::srgba(0.0000, 0.0000, 1.0000, 1.0000)),
        "teal" => Some(Color::srgba(0.0000, 0.5020, 0.5020, 1.0000)),
        "aqua" => Some(Color::srgba(0.0000, 1.0000, 1.0000, 1.0000)),

        // CSS Level 2 values
        "orange" => Some(Color::srgba(1.0000, 0.6471, 0.0000, 1.0000)),

        // CSS Level 3 values
        "aliceblue" => Some(Color::srgba(0.9412, 0.9725, 1.0000, 1.0000)),
        "antiquewhite" => Some(Color::srgba(0.9804, 0.9216, 0.8431, 1.0000)),
        "aquamarine" => Some(Color::srgba(0.4980, 1.0000, 0.8314, 1.0000)),
        "azure" => Some(Color::srgba(0.9412, 1.0000, 1.0000, 1.0000)),
        "beige" => Some(Color::srgba(0.9608, 0.9608, 0.8627, 1.0000)),
        "bisque" => Some(Color::srgba(1.0000, 0.8941, 0.7686, 1.0000)),
        "blanchedalmond" => Some(Color::srgba(1.0000, 0.9216, 0.8039, 1.0000)),
        "blueviolet" => Some(Color::srgba(0.5412, 0.1686, 0.8863, 1.0000)),
        "brown" => Some(Color::srgba(0.6471, 0.1647, 0.1647, 1.0000)),
        "burlywood" => Some(Color::srgba(0.8706, 0.7216, 0.5294, 1.0000)),
        "cadetblue" => Some(Color::srgba(0.3725, 0.6196, 0.6275, 1.0000)),
        "chartreuse" => Some(Color::srgba(0.4980, 1.0000, 0.0000, 1.0000)),
        "chocolate" => Some(Color::srgba(0.8235, 0.4118, 0.1176, 1.0000)),
        "coral" => Some(Color::srgba(1.0000, 0.4980, 0.3137, 1.0000)),
        "cornflowerblue" => Some(Color::srgba(0.3922, 0.5843, 0.9294, 1.0000)),
        "cornsilk" => Some(Color::srgba(1.0000, 0.9725, 0.8627, 1.0000)),
        "crimson" => Some(Color::srgba(0.8627, 0.0784, 0.2353, 1.0000)),
        "cyan" => Some(Color::srgba(0.0000, 1.0000, 1.0000, 1.0000)),
        "darkblue" => Some(Color::srgba(0.0000, 0.0000, 0.5451, 1.0000)),
        "darkcyan" => Some(Color::srgba(0.0000, 0.5451, 0.5451, 1.0000)),
        "darkgoldenrod" => Some(Color::srgba(0.7216, 0.5255, 0.0431, 1.0000)),
        "darkgray" => Some(Color::srgba(0.6627, 0.6627, 0.6627, 1.0000)),
        "darkgreen" => Some(Color::srgba(0.0000, 0.3922, 0.0000, 1.0000)),
        "darkgrey" => Some(Color::srgba(0.6627, 0.6627, 0.6627, 1.0000)),
        "darkkhaki" => Some(Color::srgba(0.7412, 0.7176, 0.4196, 1.0000)),
        "darkmagenta" => Some(Color::srgba(0.5451, 0.0000, 0.5451, 1.0000)),
        "darkolivegreen" => Some(Color::srgba(0.3333, 0.4196, 0.1843, 1.0000)),
        "darkorange" => Some(Color::srgba(1.0000, 0.5490, 0.0000, 1.0000)),
        "darkorchid" => Some(Color::srgba(0.6000, 0.1961, 0.8000, 1.0000)),
        "darkred" => Some(Color::srgba(0.5451, 0.0000, 0.0000, 1.0000)),
        "darksalmon" => Some(Color::srgba(0.9137, 0.5882, 0.4784, 1.0000)),
        "darkseagreen" => Some(Color::srgba(0.5608, 0.7373, 0.5608, 1.0000)),
        "darkslateblue" => Some(Color::srgba(0.2824, 0.2392, 0.5451, 1.0000)),
        "darkslategray" => Some(Color::srgba(0.1843, 0.3098, 0.3098, 1.0000)),
        "darkslategrey" => Some(Color::srgba(0.1843, 0.3098, 0.3098, 1.0000)),
        "darkturquoise" => Some(Color::srgba(0.0000, 0.8078, 0.8196, 1.0000)),
        "darkviolet" => Some(Color::srgba(0.5804, 0.0000, 0.8275, 1.0000)),
        "deeppink" => Some(Color::srgba(1.0000, 0.0784, 0.5765, 1.0000)),
        "deepskyblue" => Some(Color::srgba(0.0000, 0.7490, 1.0000, 1.0000)),
        "dimgray" => Some(Color::srgba(0.4118, 0.4118, 0.4118, 1.0000)),
        "dimgrey" => Some(Color::srgba(0.4118, 0.4118, 0.4118, 1.0000)),
        "dodgerblue" => Some(Color::srgba(0.1176, 0.5647, 1.0000, 1.0000)),
        "firebrick" => Some(Color::srgba(0.6980, 0.1333, 0.1333, 1.0000)),
        "floralwhite" => Some(Color::srgba(1.0000, 0.9804, 0.9412, 1.0000)),
        "forestgreen" => Some(Color::srgba(0.1333, 0.5451, 0.1333, 1.0000)),
        "gainsboro" => Some(Color::srgba(0.8627, 0.8627, 0.8627, 1.0000)),
        "ghostwhite" => Some(Color::srgba(0.9725, 0.9725, 1.0000, 1.0000)),
        "gold" => Some(Color::srgba(1.0000, 0.8431, 0.0000, 1.0000)),
        "goldenrod" => Some(Color::srgba(0.8549, 0.6471, 0.1255, 1.0000)),
        "greenyellow" => Some(Color::srgba(0.6784, 1.0000, 0.1843, 1.0000)),
        "grey" => Some(Color::srgba(0.5020, 0.5020, 0.5020, 1.0000)),
        "honeydew" => Some(Color::srgba(0.9412, 1.0000, 0.9412, 1.0000)),
        "hotpink" => Some(Color::srgba(1.0000, 0.4118, 0.7059, 1.0000)),
        "indianred" => Some(Color::srgba(0.8039, 0.3608, 0.3608, 1.0000)),
        "indigo" => Some(Color::srgba(0.2941, 0.0000, 0.5098, 1.0000)),
        "ivory" => Some(Color::srgba(1.0000, 1.0000, 0.9412, 1.0000)),
        "khaki" => Some(Color::srgba(0.9412, 0.9020, 0.5490, 1.0000)),
        "lavender" => Some(Color::srgba(0.9020, 0.9020, 0.9804, 1.0000)),
        "lavenderblush" => Some(Color::srgba(1.0000, 0.9412, 0.9608, 1.0000)),
        "lawngreen" => Some(Color::srgba(0.4863, 0.9882, 0.0000, 1.0000)),
        "lemonchiffon" => Some(Color::srgba(1.0000, 0.9804, 0.8039, 1.0000)),
        "lightblue" => Some(Color::srgba(0.6784, 0.8471, 0.9020, 1.0000)),
        "lightcoral" => Some(Color::srgba(0.9412, 0.5020, 0.5020, 1.0000)),
        "lightcyan" => Some(Color::srgba(0.8784, 1.0000, 1.0000, 1.0000)),
        "lightgoldenrodyellow" => Some(Color::srgba(0.9804, 0.9804, 0.8235, 1.0000)),
        "lightgray" => Some(Color::srgba(0.8275, 0.8275, 0.8275, 1.0000)),
        "lightgreen" => Some(Color::srgba(0.5647, 0.9333, 0.5647, 1.0000)),
        "lightgrey" => Some(Color::srgba(0.8275, 0.8275, 0.8275, 1.0000)),
        "lightpink" => Some(Color::srgba(1.0000, 0.7137, 0.7569, 1.0000)),
        "lightsalmon" => Some(Color::srgba(1.0000, 0.6275, 0.4784, 1.0000)),
        "lightseagreen" => Some(Color::srgba(0.1255, 0.6980, 0.6667, 1.0000)),
        "lightskyblue" => Some(Color::srgba(0.5294, 0.8078, 0.9804, 1.0000)),
        "lightslategray" => Some(Color::srgba(0.4667, 0.5333, 0.6000, 1.0000)),
        "lightslategrey" => Some(Color::srgba(0.4667, 0.5333, 0.6000, 1.0000)),
        "lightsteelblue" => Some(Color::srgba(0.6902, 0.7686, 0.8706, 1.0000)),
        "lightyellow" => Some(Color::srgba(1.0000, 1.0000, 0.8784, 1.0000)),
        "limegreen" => Some(Color::srgba(0.1961, 0.8039, 0.1961, 1.0000)),
        "linen" => Some(Color::srgba(0.9804, 0.9412, 0.9020, 1.0000)),
        "magenta" => Some(Color::srgba(1.0000, 0.0000, 1.0000, 1.0000)),
        "mediumaquamarine" => Some(Color::srgba(0.4000, 0.8039, 0.6667, 1.0000)),
        "mediumblue" => Some(Color::srgba(0.0000, 0.0000, 0.8039, 1.0000)),
        "mediumorchid" => Some(Color::srgba(0.7294, 0.3333, 0.8275, 1.0000)),
        "mediumpurple" => Some(Color::srgba(0.5765, 0.4392, 0.8588, 1.0000)),
        "mediumseagreen" => Some(Color::srgba(0.2353, 0.7020, 0.4431, 1.0000)),
        "mediumslateblue" => Some(Color::srgba(0.4824, 0.4078, 0.9333, 1.0000)),
        "mediumspringgreen" => Some(Color::srgba(0.0000, 0.9804, 0.6039, 1.0000)),
        "mediumturquoise" => Some(Color::srgba(0.2824, 0.8196, 0.8000, 1.0000)),
        "mediumvioletred" => Some(Color::srgba(0.7804, 0.0824, 0.5216, 1.0000)),
        "midnightblue" => Some(Color::srgba(0.0980, 0.0980, 0.4392, 1.0000)),
        "mintcream" => Some(Color::srgba(0.9608, 1.0000, 0.9804, 1.0000)),
        "mistyrose" => Some(Color::srgba(1.0000, 0.8941, 0.8824, 1.0000)),
        "moccasin" => Some(Color::srgba(1.0000, 0.8941, 0.7098, 1.0000)),
        "navajowhite" => Some(Color::srgba(1.0000, 0.8706, 0.6784, 1.0000)),
        "oldlace" => Some(Color::srgba(0.9922, 0.9608, 0.9020, 1.0000)),
        "olivedrab" => Some(Color::srgba(0.4196, 0.5569, 0.1373, 1.0000)),
        "orangered" => Some(Color::srgba(1.0000, 0.2706, 0.0000, 1.0000)),
        "orchid" => Some(Color::srgba(0.8549, 0.4392, 0.8392, 1.0000)),
        "palegoldenrod" => Some(Color::srgba(0.9333, 0.9098, 0.6667, 1.0000)),
        "palegreen" => Some(Color::srgba(0.5961, 0.9843, 0.5961, 1.0000)),
        "paleturquoise" => Some(Color::srgba(0.6863, 0.9333, 0.9333, 1.0000)),
        "palevioletred" => Some(Color::srgba(0.8588, 0.4392, 0.5765, 1.0000)),
        "papayawhip" => Some(Color::srgba(1.0000, 0.9373, 0.8353, 1.0000)),
        "peachpuff" => Some(Color::srgba(1.0000, 0.8549, 0.7255, 1.0000)),
        "peru" => Some(Color::srgba(0.8039, 0.5216, 0.2471, 1.0000)),
        "pink" => Some(Color::srgba(1.0000, 0.7529, 0.7961, 1.0000)),
        "plum" => Some(Color::srgba(0.8667, 0.6275, 0.8667, 1.0000)),
        "powderblue" => Some(Color::srgba(0.6902, 0.8784, 0.9020, 1.0000)),
        "rosybrown" => Some(Color::srgba(0.7373, 0.5608, 0.5608, 1.0000)),
        "royalblue" => Some(Color::srgba(0.2549, 0.4118, 0.8824, 1.0000)),
        "saddlebrown" => Some(Color::srgba(0.5451, 0.2706, 0.0745, 1.0000)),
        "salmon" => Some(Color::srgba(0.9804, 0.5020, 0.4471, 1.0000)),
        "sandybrown" => Some(Color::srgba(0.9569, 0.6431, 0.3765, 1.0000)),
        "seagreen" => Some(Color::srgba(0.1804, 0.5451, 0.3412, 1.0000)),
        "seashell" => Some(Color::srgba(1.0000, 0.9608, 0.9333, 1.0000)),
        "sienna" => Some(Color::srgba(0.6275, 0.3216, 0.1765, 1.0000)),
        "skyblue" => Some(Color::srgba(0.5294, 0.8078, 0.9216, 1.0000)),
        "slateblue" => Some(Color::srgba(0.4157, 0.3529, 0.8039, 1.0000)),
        "slategray" => Some(Color::srgba(0.4392, 0.5020, 0.5647, 1.0000)),
        "slategrey" => Some(Color::srgba(0.4392, 0.5020, 0.5647, 1.0000)),
        "snow" => Some(Color::srgba(1.0000, 0.9804, 0.9804, 1.0000)),
        "springgreen" => Some(Color::srgba(0.0000, 1.0000, 0.4980, 1.0000)),
        "steelblue" => Some(Color::srgba(0.2745, 0.5098, 0.7059, 1.0000)),
        "tan" => Some(Color::srgba(0.8235, 0.7059, 0.5490, 1.0000)),
        "thistle" => Some(Color::srgba(0.8471, 0.7490, 0.8471, 1.0000)),
        "tomato" => Some(Color::srgba(1.0000, 0.3882, 0.2784, 1.0000)),
        "transparent" => Some(Color::srgba(0.0000, 0.0000, 0.0000, 0.0000)),
        "turquoise" => Some(Color::srgba(0.2510, 0.8784, 0.8157, 1.0000)),
        "violet" => Some(Color::srgba(0.9333, 0.5098, 0.9333, 1.0000)),
        "wheat" => Some(Color::srgba(0.9608, 0.8706, 0.7020, 1.0000)),
        "whitesmoke" => Some(Color::srgba(0.9608, 0.9608, 0.9608, 1.0000)),
        "yellowgreen" => Some(Color::srgba(0.6039, 0.8039, 0.1961, 1.0000)),

        // CSS Level 4 values
        "rebeccapurple" => Some(Color::srgba(0.4000, 0.2000, 0.6000, 1.0000)),
        _ => None,
    }
}
