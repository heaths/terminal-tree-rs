// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use clap::Parser;
use terminal_tree::{
    clap::{default_colors, default_indentation, range, ColorValue},
    TreeBuilder,
};

fn main() {
    let args = Args::parse();
    let builder = TreeBuilder::default()
        .colors(args.colors)
        .indentation(args.indent);

    for i in 0..8 {
        builder.indent(i);
        println!();
    }
}

#[derive(Parser)]
#[command(about = "Shows a simple tree of branches and leaves.")]
struct Args {
    /// Optional array of colors to use repeatedly for branch lines.
    #[arg(long, value_delimiter = ',', default_values_t = default_colors())]
    colors: Vec<ColorValue>,

    /// Optional level of indentation.
    #[arg(long, value_parser = indentation, default_value_t = default_indentation())]
    indent: u8,
}

fn indentation(arg: &str) -> Result<u8, String> {
    range(arg, 2, u8::MAX)
}
