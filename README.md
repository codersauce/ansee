# Ansee

**Ansee** is a command-line tool that converts ANSI-escaped text to PNG images. It's designed to capture the colorful output of terminal commands and preserve them as shareable images.

## Features

- **Accurate ANSI Parsing:** Ansee utilizes the `ansi-parser` crate to correctly interpret various ANSI escape codes, including those for colors, styles, and cursor movements.
- **True Color Support:** Handles 24-bit RGB color codes, ensuring faithful reproduction of terminal colors.
- **Font Rendering:** Employs `ab_glyph` for high-quality font rendering, using the included Fantasque Sans Mono Nerd Font.
- **Image Generation:** Leverages the `image` crate to create PNG output.

## Installation

1.  **Rust Environment:** Make sure you have Rust and Cargo installed. If not, follow the instructions at [rustup.rs](https://www.google.com/url?sa=E&source=gmail&q=https://rustup.rs/).

2.  **Clone the Repository:**

    ```bash
    git clone https://github.com/codersauce/ansee.git
    cd ansee
    ```

3.  **Build and Install:**

    ```bash
    cargo build --release
    cargo install --path .
    ```

## Usage

Pipe the output of any command with ANSI escape codes to `ansee`:

```bash
ls -l --color=always | ansee
```

This will generate an image named `output.png` in the current directory.

## Examples

Here are a few examples of how you can use Ansee:

- **Capture `ls` output:**

  ```bash
  ls -l --color=always | ansee
  ```

- **Save a colorful `neofetch` output:**

  ```bash
  neofetch | ansee
  ```

- **Preserve the output of a command with syntax highlighting:**

  ```bash
  bat src/main.rs | ansee
  ```

## Customization (Future)

Currently, Ansee provides basic functionality. Future enhancements may include:

- **Configurable Output:** Options to specify the output filename, image dimensions, and background color.
- **Font Selection:** Allow users to choose different fonts or provide their own.
- **Themes:** Support for predefined color themes or user-defined palettes.

## Contributing

Contributions are welcome\! Feel free to open issues for bug reports or feature requests. Pull requests are appreciated for bug fixes, new features, and improvements to the codebase.

## License

This project is licensed under the MIT License - see the [LICENSE](https://www.google.com/url?sa=E&source=gmail&q=LICENSE) file for details.
