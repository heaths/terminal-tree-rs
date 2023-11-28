// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use clap::Parser;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use terminal_tree::{
    clap::{default_colors, default_indentation, range, ColorValue},
    TreeBranch, TreeBuilder,
};

fn main() -> io::Result<()> {
    let args = Args::parse();
    let builder = TreeBuilder::default()
        .colors(args.colors)
        .indentation(args.indent);

    let path = fs::canonicalize(args.path)?;
    let branch = builder.branch(path.display());
    visit_dir(&branch, &path)
}

fn visit_dir(branch: &TreeBranch<std::path::Display<'_>>, dir: &Path) -> io::Result<()> {
    let branch = branch.branch(dir.display());
    println!("{}", branch);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit_dir(&branch, &path)?;
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(about = "Shows a simple tree of branches and leaves.")]
struct Args {
    /// The directory to enumerate.
    #[arg(default_value = ".")]
    path: PathBuf,

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
