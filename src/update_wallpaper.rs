use text_to_png::TextRenderer;

pub fn update_wallpaper(deadline: &str, font_size: u8, font_color: String) {
    let file_path = generate_wallpaper(deadline, font_size, font_color);

    // Sets the wallpaper for the current desktop from a URL.
    wallpaper::set_mode(wallpaper::Mode::Center).unwrap();
    wallpaper::set_from_path(&file_path).unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}

fn generate_wallpaper(deadline: &str, font_size: u8, font_color: String) -> String {
    let renderer = TextRenderer::try_new_with_ttf_font_data(include_bytes!(
        "../assets/fonts/Poppins-Black.ttf"
    ))
    .unwrap();

    let text_png = renderer
        .render_text_to_png_data(deadline, font_size, font_color.as_str())
        .unwrap();

    let text_image = image::load_from_memory(&text_png.data).unwrap();

    let mut background = image::open("background.png").unwrap();

    // 50% Background Image width or height - 50% Text Image width or height
    // To Center the text both horizontally and vertically
    let x = background.width() / 2 - text_png.size.width / 2;
    let y = background.height() / 2 - text_png.size.height / 2;

    image::imageops::overlay(&mut background, &text_image, x as i64, y as i64);

    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let file_path = cache_dir.join("result.png");
    let file_path = file_path.to_str().unwrap().to_owned();

    background
        .save(&file_path)
        .expect("Couldn't save result.png");

    file_path
}
