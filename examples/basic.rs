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
