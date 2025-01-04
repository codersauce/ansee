use std::{
    collections::HashMap,
    io::{self, Read, Write},
};

use ansee::{
    cli::{Cli, Commands},
    draw_image, Font,
};
use clap::Parser;
use dafont::{get_font_name, FcFontCache, FcPattern, PatternMatch};
use image::ImageFormat;

fn read_stdin() -> anyhow::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            let input = cli.input.map_or_else(read_stdin, |path| {
                let mut buf = String::new();
                std::fs::File::open(path)?.read_to_string(&mut buf)?;
                Ok(buf)
            })?;

            let fonts = Font {
                name: cli.font,
                size: cli.font_size.unwrap_or(20.0),
                line_height: cli.line_height.unwrap_or(1.1),
            };

            let image = draw_image(&input, fonts)?;

            if let Some(path) = cli.output {
                image.save(path)?;
            } else {
                let mut buffer = std::io::Cursor::new(Vec::new());
                image.write_to(&mut buffer, ImageFormat::Png)?;

                io::stdout().write_all(&buffer.into_inner())?;
            }
        }
        Some(Commands::ListFonts) => {
            list_fonts();
        }
    }

    Ok(())
}

fn list_fonts() {
    let cache = FcFontCache::build();
    let fonts = cache.query_all(&FcPattern {
        monospace: PatternMatch::True,
        ..Default::default()
    });

    let mut font_by_family = HashMap::new();
    for font in fonts {
        let Some((family, name)) = get_font_name(font) else {
            eprintln!("failed to get font name for {}", font.path);
            continue;
        };

        font_by_family
            .entry(family)
            .or_insert_with(Vec::new)
            .push(name);
    }

    let mut families: Vec<_> = font_by_family.keys().collect();
    families.sort();

    for family in families {
        println!("{family}");

        let names = &font_by_family[family];
        for name in names {
            println!("  {name}");
        }

        println!();
    }
}
