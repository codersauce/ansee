use clap::{Parser, Subcommand};

/// Render ANSI escaped text to image
#[derive(Parser)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Input file to render or stdin if not present
    pub input: Option<String>,

    /// Output file to write to or stdout if not present
    #[clap(short, long)]
    pub output: Option<String>,

    /// Font used for rendering the image
    #[clap(short = 'f', long)]
    pub font: Option<String>,

    /// Font size in pixels, defaults to 20.0
    #[clap(short = 's', long)]
    pub font_size: Option<f32>,

    /// Line height in a factor of height, defaults to 1.1
    #[clap(short = 'e', long)]
    pub line_height: Option<f32>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List of font families that can be used
    ListFonts,
}
