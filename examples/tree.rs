// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use terminal_tree::TreeBuilder;

fn main() {
    let builder = TreeBuilder::default();
    for i in 0..8 {
        builder.indent(i);
        println!();
    }
}
