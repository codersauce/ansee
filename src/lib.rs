pub mod cli;
pub mod colors;

use std::path::PathBuf;

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use ansi_parser::{AnsiParser, AnsiSequence, Output};
use colors::{ANSI_MAP, MAP256};
use font_kit::{family_name::FamilyName, handle::Handle, source::SystemSource};
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};

#[derive(Debug)]
enum Command {
    Reset,
    ResetForegroundColor,
    ResetBackgroundColor,
    ReverseColor,
    SetForegroundColor(Rgba<u8>),
    SetBackgroundColor(Rgba<u8>),
    SetItalic,
    Text(String),
}

fn handle_graphics_mode(data: &[u8]) -> anyhow::Result<Option<Command>> {
    let code = data.first().copied().unwrap_or(0);
    let result = match code {
        0 => Some(Command::Reset),
        3 => Some(Command::SetItalic),
        7 => Some(Command::ReverseColor),
        30..=37 | 90..=97 => ANSI_MAP
            .get(&code)
            .copied()
            .map(Command::SetForegroundColor),
        40..=47 | 100..=107 => ANSI_MAP
            .get(&code)
            .copied()
            .map(Command::SetBackgroundColor),
        39 => Some(Command::ResetForegroundColor),
        49 => Some(Command::ResetBackgroundColor),
        38 | 48 => {
            let data = data.get(1..).unwrap_or(&[]);

            if data[0] == 2 {
                // rgb
                if code == 38 {
                    Some(Command::SetForegroundColor(Rgba([
                        data[1], data[2], data[3], 255,
                    ])))
                } else {
                    Some(Command::SetBackgroundColor(Rgba([
                        data[1], data[2], data[3], 255,
                    ])))
                }
            } else if data[0] == 5 {
                let Some(color) = MAP256.get(data[1] as usize).copied() else {
                    anyhow::bail!("Invalid color code: {}", data[1]);
                };

                if code == 38 {
                    Some(Command::SetForegroundColor(color))
                } else {
                    Some(Command::SetBackgroundColor(color))
                }
            } else {
                anyhow::bail!("Invalid color code: {}", data[0]);
            }
        }
        _ => None,
    };

    Ok(result)
}

fn extract_text(commands: &[Command]) -> String {
    commands
        .iter()
        .filter_map(|cmd| match cmd {
            Command::Text(s) => Some(s.clone()),
            _ => None,
        })
        .collect()
}

#[derive(Debug)]
pub struct Fonts {
    pub main: Option<PathBuf>,
    pub size: f32,
    pub line_height: f32,
}

pub fn draw_image(input: &str, font_info: Fonts) -> anyhow::Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let commands = parse_ansi(input);
    let text = extract_text(&commands);

    let source = SystemSource::new();
    let handle = match font_info.main {
        Some(main) => source.select_best_match(
            &[FamilyName::Title(main.to_string_lossy().to_string())],
            &Default::default(),
        )?,
        None => source.select_best_match(&[FamilyName::Monospace], &Default::default())?,
    };
    let font_data = match handle {
        Handle::Path { path, .. } => std::fs::read(path)?,
        Handle::Memory { bytes, .. } => bytes.to_vec(),
    };
    let font = FontRef::try_from_slice(&font_data)?;
    let font_size = font_info.size;

    let scale = PxScale::from(font_size);
    let scaled_font = font.as_scaled(scale);

    let line_height = font_size * font_info.line_height;
    let max_width = text.lines().map(|line| line.len()).max().unwrap_or(0);
    let glyph = font.glyph_id('m');
    let char_width = scaled_font.h_advance(glyph);
    let width = max_width as f32 * char_width;
    let height = text.lines().count() as f32 * line_height;

    let mut image = RgbaImage::new(width.ceil() as u32, height.ceil() as u32);

    let mut cx: f32 = 0.0;
    let mut cy: f32 = 0.0;
    let mut fg_color = Rgba([255, 255, 255, 255]);
    let mut bg_color = Rgba([0, 0, 0, 255]);

    for command in commands {
        match command {
            Command::SetForegroundColor(color) => fg_color = color,
            Command::SetBackgroundColor(color) => bg_color = color,
            Command::Text(s) => {
                for c in s.chars() {
                    if c == '\n' {
                        cx = 0.0;
                        cy += line_height;
                        continue;
                    }

                    let glyph = font.glyph_id(c);
                    let char_width = scaled_font.h_advance(glyph);

                    let rect = Rect::at(cx.round() as i32, cy.round() as i32)
                        .of_size(char_width.ceil() as u32, line_height.ceil() as u32);
                    draw_filled_rect_mut(&mut image, rect, bg_color);
                    draw_text_mut(
                        &mut image,
                        fg_color,
                        cx as i32,
                        cy as i32,
                        scale,
                        &font,
                        &c.to_string(),
                    );
                    cx += char_width;
                }
            }
            Command::Reset => {
                fg_color = Rgba([255, 255, 255, 255]);
                bg_color = Rgba([0, 0, 0, 255]);
            }
            _ => {}
        }
    }

    Ok(image)
}

fn parse_ansi(text: &str) -> Vec<Command> {
    let mut commands = vec![];
    for item in text.ansi_parse() {
        match item {
            Output::TextBlock(s) => commands.push(Command::Text(s.to_string())),
            Output::Escape(ansi_sequence) => match ansi_sequence {
                AnsiSequence::SetGraphicsMode(mode) => match handle_graphics_mode(&mode) {
                    Ok(Some(cmd)) => commands.push(cmd),
                    Ok(None) => println!("Skipped graphics mode: {:?}", mode),
                    Err(e) => println!("Error: {:?}", e),
                },
                _ => {
                    println!("Escape: {:?}", ansi_sequence);
                }
            },
        }
    }

    commands
}
