// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

// cspell:ignore rrggbb

use ::clap::{error, Arg, Command, Error, ValueEnum};
use clap::builder::{PossibleValue, TypedValueParser, ValueParserFactory};
use crossterm::style;

use crate::TreeBuilder;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorValue(style::Color);

impl std::ops::Deref for ColorValue {
    type Target = style::Color;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ColorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: String = self.into();
        write!(f, "{value}")
    }
}

impl From<style::Color> for ColorValue {
    fn from(value: style::Color) -> Self {
        ColorValue(value)
    }
}

impl From<&style::Color> for ColorValue {
    fn from(value: &style::Color) -> Self {
        ColorValue(*value)
    }
}

impl From<ColorValue> for style::Color {
    fn from(value: ColorValue) -> Self {
        value.0
    }
}

impl From<ColorValue> for String {
    fn from(value: ColorValue) -> Self {
        String::from(&value)
    }
}

impl From<&ColorValue> for String {
    fn from(value: &ColorValue) -> Self {
        match value.0 {
            style::Color::Black => "black".to_string(),
            style::Color::DarkGrey => "dark_grey".to_string(),
            style::Color::Red => "red".to_string(),
            style::Color::DarkRed => "dark_red".to_string(),
            style::Color::Green => "green".to_string(),
            style::Color::DarkGreen => "dark_green".to_string(),
            style::Color::Yellow => "yellow".to_string(),
            style::Color::DarkYellow => "dark_yellow".to_string(),
            style::Color::Blue => "blue".to_string(),
            style::Color::DarkBlue => "dark_blue".to_string(),
            style::Color::Magenta => "magenta".to_string(),
            style::Color::DarkMagenta => "dark_magenta".to_string(),
            style::Color::Cyan => "cyan".to_string(),
            style::Color::DarkCyan => "dark_cyan".to_string(),
            style::Color::White => "white".to_string(),
            style::Color::Grey => "grey".to_string(),
            style::Color::Rgb { r, g, b } => format!("#{:02x}{:02x}{:02x}", r, g, b),
            _ => panic!("not supported"),
        }
    }
}

impl ValueParserFactory for ColorValue {
    type Parser = ColorValueParser;

    fn value_parser() -> Self::Parser {
        ColorValueParser {}
    }
}

#[derive(Clone)]
pub struct ColorValueParser {}

impl ColorValueParser {
    pub fn default_values(values: &'static [style::Color]) -> impl Iterator<Item = ColorValue> {
        values.iter().map(ColorValue::from)
    }
}

impl TypedValueParser for ColorValueParser {
    type Value = ColorValue;

    fn parse_ref(
        &self,
        cmd: &Command,
        arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, error::Error> {
        let value = value
            .to_str()
            .ok_or_else(|| error::Error::new(error::ErrorKind::InvalidValue).with_cmd(cmd))?;

        if let Some(hex) = value.strip_prefix('#') {
            if hex.is_ascii() && hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16);
                let g = u8::from_str_radix(&hex[2..4], 16);
                let b = u8::from_str_radix(&hex[4..6], 16);

                if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
                    return Ok(style::Color::Rgb { r, g, b }.into());
                }
            }
        }

        style::Color::try_from(value)
            .map(|v| v.into())
            .map_err(|err| error::Error::new(error::ErrorKind::InvalidValue).with_cmd(cmd))
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        static VARIANTS: &[&str] = &[
            "black",
            "dark_grey",
            "red",
            "dark_red",
            "green",
            "dark_green",
            "yellow",
            "dark_yellow",
            "blue",
            "dark_blue",
            "magenta",
            "dark_magenta",
            "cyan",
            "dark_cyan",
            "white",
            "grey",
            "#rrggbb",
        ];

        Some(Box::new(VARIANTS.iter().map(PossibleValue::new)))
    }
}
