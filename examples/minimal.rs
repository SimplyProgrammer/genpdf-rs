// SPDX-FileCopyrightText: 2021 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example generates a minimal PDF document and writes it to the path that was passed as the
//! first command-line argument.  The size of the generated document should be 2.0 KB.
//!
//! You may have to adapt the `FONT_DIRS` and `DEFAULT_FONT_NAME` constants for your system so that
//! these files exist:
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Regular.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Bold.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-Italic.ttf`
//! - `{FONT_DIR}/{DEFAULT_FONT_NAME}-BoldItalic.ttf`
//! <br>However it will also accept the variant without "-" or with space instead of "-". Also variants with and without spaces will be checked and/or variants with suffix shorthands R, B, I, BI or (nothing), B, I, Z
//! <br>But it is recommended to follow the naming convention above.
//!
//! These fonts must be metrically identical to the built-in PDF sans-serif font (Helvetica/Arial).

use std::env;

use genpdf::{elements, fonts};

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];
const DEFAULT_FONT_NAME: &'static str = "LiberationSans";

fn main() {
    let mut args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        args = vec!["./examples/minimal.pdf".into()]
    }
    let output_file = &args[0];

    let font_dir = FONT_DIRS // Not really necessary from_files will search sys fot dir by default...
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .unwrap_or(&&"");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_title("genpdf Demo Document");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    doc.push(elements::Text::new("Minimal PDF document"));

    doc.render_to_file(output_file)
        .expect("Failed to write output file");
}
