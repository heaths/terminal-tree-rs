// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![allow(dead_code, unused_imports, unused_variables)]

use crossterm::{
    style::{Color, Stylize},
    terminal,
    tty::IsTty,
};
use std::{fmt::Display, io, vec};
#[cfg(feature = "clap")]
pub mod clap;

const BRANCH: &str = "\u{251c}";
const BRANCH_LAST: &str = "\u{2514}";
const VERTICAL: &str = "\u{2502}";

pub struct TreeBuilder {
    _colors: Vec<Color>,
    _indentation: u8,
}

impl TreeBuilder {
    pub(crate) const COLORS: &[Color] = &[
        Color::Rgb {
            r: 0xff,
            g: 0xd7,
            b: 0x00,
        },
        Color::Rgb {
            r: 0xda,
            g: 0x70,
            b: 0xd6,
        },
        Color::Rgb {
            r: 0x17,
            g: 0x9f,
            b: 0xff,
        },
    ];
    pub(crate) const INDENTATION: u8 = 2;

    pub fn colors<I, C>(mut self, colors: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Color>,
    {
        self._colors = colors.into_iter().map(C::into).collect::<Vec<_>>().to_vec();
        self
    }

    pub fn indentation(mut self, indentation: u8) -> Self {
        self._indentation = indentation;
        self
    }

    pub fn tty(self, tty: bool) -> Self {
        crossterm::style::force_color_output(tty);
        self
    }

    pub fn indent(&self, level: u8) {
        for i in 1..level {
            let color = self._colors.iter().cycle().nth((i - 1) as usize).unwrap();
            print!(
                "{}{}",
                VERTICAL.with(*color),
                String::from_iter(vec![" "; (self._indentation - 1) as usize]),
            )
        }
    }
}

impl Default for TreeBuilder {
    fn default() -> Self {
        TreeBuilder {
            // Default colors for Visual Studio Code.
            _colors: Self::COLORS.to_vec(),
            _indentation: Self::INDENTATION,
        }
    }
}

pub struct TreeItem<'a> {
    builder: &'a TreeBuilder,
    level: u8,
}

impl<'a> TreeItem<'a> {
    pub fn branch<'b, I>(&self, items: I) -> Self
    where
        I: Iterator<Item = &'b str>,
    {
        todo!()
    }
}

impl<'a> Display for TreeItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> Drop for TreeItem<'a> {
    fn drop(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }
}
