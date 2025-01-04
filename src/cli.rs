use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
/// Render text as an image
pub struct Args {
    /// Input file to render or stdin if not present
    pub input: Option<PathBuf>,

    /// Output file to write to or stdout if not present
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Font used for rendering the image
    #[clap(short = 'f', long)]
    pub font: Option<PathBuf>,

    /// Font used for rendering italic text, fallsback to font
    #[clap(short = 'i', long)]
    pub font_italic: Option<PathBuf>,

    /// Font used for rendering bold text, fallsback to font
    #[clap(short = 'b', long)]
    pub font_bold: Option<PathBuf>,

    /// Font used for rendering bold italic text, fallsback to font
    #[clap(short = 'x', long)]
    pub font_bold_italic: Option<PathBuf>,

    /// Font size in pixels
    #[clap(short = 's', long)]
    pub font_size: Option<f32>,

    /// Line height in a factor of height, defaults to 1.1
    #[clap(short = 'e', long)]
    pub line_height: Option<f32>,
}
