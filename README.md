# ansee

A command-line tool for converting ANSI-escaped text to images. Supports 256 colors, RGB colors, and custom fonts.

## Features

- Convert ANSI-escaped text to PNG images
- Support for standard ANSI colors (16 colors)
- Extended color support (256 colors)
- True color support (RGB)
- Custom font support
- Configurable font size and line height
- Input from file or stdin
- Output to file or stdout

## Installation

Make sure you have Rust installed on your system. Then:

```bash
# Clone the repository
git clone https://github.com/codersauce/ansee
cd ansee

# Build and install
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Convert ANSI text from a file
ansee input.txt -o output.png

# Convert ANSI text from stdin
echo -e "\033[31mHello\033[0m \033[32mWorld\033[0m" | ansee

# Use a specific font
ansee input.txt -f "Fira Code" -o output.png

# Customize font size and line height
ansee input.txt -s 24 -e 1.5 -o output.png
```

### Available Options

```
Usage: ansee [OPTIONS] [INPUT] [COMMAND]

Arguments:
  [INPUT]  Input file to render or stdin if not present

Options:
  -o, --output <OUTPUT>        Output file to write to or stdout if not present
  -f, --font <FONT>           Font used for rendering the image
  -s, --font-size <SIZE>      Font size in pixels [default: 20.0]
  -e, --line-height <HEIGHT>  Line height in a factor of height [default: 1.1]
  -h, --help                  Print help
  -V, --version              Print version

Commands:
  list-fonts  List available font families that can be used
  help        Print this message or help for given subcommand
```

### Font Management

To list all available monospace fonts on your system:

```bash
ansee list-fonts
```

## ANSI Support

The tool supports the following ANSI escape sequences:

- Standard colors (30-37, 40-47)
- Bright colors (90-97, 100-107)
- 256 colors (38;5;n, 48;5;n)
- RGB colors (38;2;r;g;b, 48;2;r;g;b)
- Reset (0)
- Italic (3)
- Reverse color (7)

## Examples

### Basic Text Coloring

```bash
echo -e "\033[31mRed Text\033[0m \033[42mGreen Background\033[0m" | ansee -o colors.png
```

### Using 256 Colors

```bash
echo -e "\033[38;5;82mBright Green\033[0m \033[48;5;25mBlue Background\033[0m" | ansee -o 256colors.png
```

### Using RGB Colors

```bash
echo -e "\033[38;2;255;128;0mOrange Text\033[0m" | ansee -o rgb.png
```

## Library Usage

You can use ansee as a library in your Rust projects. Add it to your `Cargo.toml`:

```toml
[dependencies]
ansee = { git = "https://github.com/codersauce/ansee" }
```

### Example

```rust
use ansee::draw_image;

fn main() -> anyhow::Result<()> {
    // Create ANSI-escaped text
    let input = "\x1b[31mHello\x1b[0m \x1b[32mWorld\x1b[0m";

    // Generate the image with the specified system font, you can use a string
    // with font_name:font_size:line_height and convert it to a Font with
    // `.try_into` like below
    let image = draw_image(input, "FantasqueSansM Nerd Font:18".try_into()?)?;

    // Save to file
    image.save("output.png")?;

    Ok(())
}
```

### API Reference

#### `draw_image`

```rust
pub fn draw_image(
    input: &str,
    font_info: Font
) -> anyhow::Result<ImageBuffer<Rgba<u8>, Vec<u8>>>
```

Converts ANSI-escaped text to an image. Returns an `ImageBuffer` that can be saved or manipulated further.

#### `Font` struct

```rust
pub struct Font {
    pub name: Option<String>,     // Font name (uses system monospace if None)
    pub size: f32,                // Font size in pixels
    pub line_height: f32,         // Line height as a factor of font size
}
```

Or you can convert from a colon delimited `&str`, like `FantasqueSansM:18.0:1.2` and `try_into`:

```rust
draw_image("\x1b[31mHello\x1b[0m \x1b[32mWorld\x1b[0m", "FantasqueSansM:18.0:1.2".try_into()?);
```

The library provides full access to the same functionality as the CLI tool, allowing you to:

- Render ANSI-escaped text with color support
- Customize fonts and text appearance
- Generate images programmatically
- Integrate with existing Rust applications

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Uses [ab_glyph](https://crates.io/crates/ab_glyph) for font rendering
- Uses [image](https://crates.io/crates/image) for image processing
- Uses [ansi-parser](https://crates.io/crates/ansi-parser) for ANSI escape sequence parsing
