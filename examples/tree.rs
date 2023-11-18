// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use clap::Parser;
use terminal_tree::{
    clap::{ColorValue, ColorValueParser},
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
    #[arg(long, value_delimiter = ',', default_values_t = ColorValueParser::default_values(TreeBuilder::COLORS))]
    colors: Vec<ColorValue>,

    /// Optional level of indentation.
    #[arg(long, default_value_t = TreeBuilder::INDENTATION)]
    indent: u8,
}
