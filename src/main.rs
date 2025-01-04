use std::io::{self, Read, Write};

use ansee::{cli::Args, draw_image, Fonts};
use clap::Parser;
use image::ImageFormat;

fn read_stdin() -> anyhow::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input = args.input.map_or_else(read_stdin, |path| {
        let mut buf = String::new();
        std::fs::File::open(path)?.read_to_string(&mut buf)?;
        Ok(buf)
    })?;

    let fonts = Fonts {
        main: args.font,
        italic: args.font_italic,
        bold: args.font_bold,
        bold_italic: args.font_bold_italic,
        size: args.font_size.unwrap_or(16.0),
        line_height: args.line_height.unwrap_or(1.1),
    };

    let image = draw_image(&input, fonts)?;

    if let Some(path) = args.output {
        image.save(path)?;
    } else {
        let mut buffer = std::io::Cursor::new(Vec::new());
        image.write_to(&mut buffer, ImageFormat::Png)?;

        io::stdout().write_all(&buffer.into_inner())?;
    }

    Ok(())
}
