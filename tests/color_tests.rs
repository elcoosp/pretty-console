//! Tests for color parsing, contrast, degradation and decorations.

use pretty_console::{
    Color, ColorLevel, ParseColorError, gradient, gradient_bg, hsv_to_rgb, palette_rgb, rainbow,
};

#[test]
fn hex_string_parsing() {
    assert_eq!(Color::from_hex_str("#7c4dff"), Ok(Color::rgb(124, 77, 255)));
    assert_eq!(Color::from_hex_str("7c4dff"), Ok(Color::rgb(124, 77, 255)));
    assert_eq!(Color::from_hex_str("#FFF"), Ok(Color::rgb(255, 255, 255)));
    assert_eq!(Color::from_hex_str("f0a"), Ok(Color::rgb(255, 0, 170)));
    assert_eq!(Color::from_hex_str(""), Err(ParseColorError::InvalidLength));
    assert_eq!(
        Color::from_hex_str("abcde"),
        Err(ParseColorError::InvalidLength)
    );
    assert_eq!(
        Color::from_hex_str("nope!"),
        Err(ParseColorError::InvalidLength)
    );
    assert_eq!(
        Color::from_hex_str("zzzzzz"),
        Err(ParseColorError::InvalidDigit('z'))
    );
    // FromStr is the same parser.
    assert_eq!("ff0000".parse::<Color>().unwrap(), Color::rgb(255, 0, 0));
}

#[test]
fn error_display() {
    let err = Color::from_hex_str("nope!").unwrap_err();
    assert_eq!(err, ParseColorError::InvalidLength);
    // ParseColorError implements std::error::Error.
    let _: &dyn std::error::Error = &err;
    assert!(err.to_string().contains("hex"));
    assert!(
        Color::from_hex_str("ab")
            .unwrap_err()
            .to_string()
            .contains("3 or 6")
    );
    assert_eq!(
        Color::from_hex_str("zzzzzz").unwrap_err(),
        ParseColorError::InvalidDigit('z')
    );
}

#[test]
fn palette_rgb_resolution() {
    assert_eq!(palette_rgb(0), (0, 0, 0));
    assert_eq!(palette_rgb(1), (205, 0, 0));
    assert_eq!(palette_rgb(15), (255, 255, 255));
    // 6x6x6 cube: index 16 is (0,0,0), index 231 is (255,255,255).
    assert_eq!(palette_rgb(16), (0, 0, 0));
    assert_eq!(palette_rgb(231), (255, 255, 255));
    assert_eq!(palette_rgb(196), (255, 0, 0)); // 16 + 36*5
    // Grayscale ramp: 232 is 8, 255 is 238.
    assert_eq!(palette_rgb(232), (8, 8, 8));
    assert_eq!(palette_rgb(255), (238, 238, 238));
}

#[test]
fn rgb_to_256_nearest() {
    // Exact cube colors map to their index.
    assert_eq!(Color::rgb(255, 0, 0).to_256(), Color::Named(196));
    assert_eq!(Color::rgb(0, 0, 255).to_256(), Color::Named(21));
    // Pure gray goes to the grayscale ramp.
    assert_eq!(Color::rgb(128, 128, 128).to_256(), Color::Named(244));
    // Off colors pick the nearest cube entry.
    assert_eq!(Color::rgb(250, 10, 10).to_256(), Color::Named(196));
}

#[test]
fn rgb_to_16_nearest() {
    // Pure RGB values land on the brightest palette entry that matches.
    assert_eq!(Color::rgb(255, 0, 0).to_16(), Color::BRIGHT_RED);
    assert_eq!(Color::rgb(0, 255, 0).to_16(), Color::BRIGHT_GREEN);
    assert_eq!(Color::rgb(255, 255, 255).to_16(), Color::BRIGHT_WHITE);
    assert_eq!(Color::rgb(0, 0, 0).to_16(), Color::BLACK);
    // Dimmer values land on the base entries.
    assert_eq!(Color::rgb(200, 0, 0).to_16(), Color::RED);
    // Named colors 0..=15 pass through.
    assert_eq!(Color::CYAN.to_16(), Color::CYAN);
    // Higher palette entries resolve through their palette RGB.
    assert_eq!(Color::indexed(196).to_16(), Color::BRIGHT_RED);
}

#[test]
fn degradation_matrix() {
    // TrueColor keeps everything.
    assert_eq!(
        Color::rgb(1, 2, 3).degrade_to(ColorLevel::TrueColor),
        Color::rgb(1, 2, 3)
    );
    // Ansi256 keeps named, converts RGB.
    assert_eq!(Color::CYAN.degrade_to(ColorLevel::Ansi256), Color::CYAN);
    assert_eq!(
        Color::rgb(255, 0, 0).degrade_to(ColorLevel::Ansi256),
        Color::Named(196)
    );
    // Ansi16 folds named 16..=255 and RGB down.
    assert_eq!(Color::CYAN.degrade_to(ColorLevel::Ansi16), Color::CYAN);
    assert_eq!(
        Color::indexed(200).degrade_to(ColorLevel::Ansi16),
        Color::BRIGHT_MAGENTA
    );
}

#[test]
fn color_levels() {
    assert_eq!(Color::RED.level(), ColorLevel::Ansi16);
    assert_eq!(Color::indexed(42).level(), ColorLevel::Ansi256);
    assert_eq!(Color::rgb(1, 2, 3).level(), ColorLevel::TrueColor);
}

#[test]
fn luminance_and_contrast() {
    assert_eq!(Color::BLACK.luminance(), 0.0);
    assert!(Color::BRIGHT_WHITE.luminance() > 0.99);
    assert!(Color::YELLOW.luminance() > Color::BLUE.luminance());

    // Dark backgrounds get white text, bright ones black text.
    assert_eq!(Color::rgb(20, 20, 20).readable_on(), Color::WHITE);
    assert_eq!(Color::YELLOW.readable_on(), Color::BLACK);
    assert_eq!(Color::BRIGHT_WHITE.readable_on(), Color::BLACK);
    assert_eq!(Color::BLACK.readable_on(), Color::WHITE);
}

#[test]
fn lerp_interpolates() {
    assert_eq!(
        Color::lerp(Color::BLACK, Color::BRIGHT_WHITE, 0.0),
        Color::BLACK
    );
    assert_eq!(
        Color::lerp(Color::BLACK, Color::BRIGHT_WHITE, 1.0),
        Color::BRIGHT_WHITE
    );
    assert_eq!(
        Color::lerp(Color::BLACK, Color::BRIGHT_WHITE, 0.5),
        Color::rgb(128, 128, 128)
    );
    // t is clamped.
    assert_eq!(
        Color::lerp(Color::BLACK, Color::BRIGHT_WHITE, 5.0),
        Color::BRIGHT_WHITE
    );
    assert_eq!(
        Color::lerp(Color::BLACK, Color::BRIGHT_WHITE, -1.0),
        Color::BLACK
    );
    // Named colors interpolate through their palette RGB.
    assert_eq!(
        Color::lerp(Color::RED, Color::RED, 0.5),
        Color::rgb(205, 0, 0)
    );
    // Named → named works through palette values.
    let mid = Color::lerp(Color::BLACK, Color::WHITE, 0.5);
    assert_eq!(mid, Color::rgb(115, 115, 115));
}

#[test]
fn hsv_to_rgb_conversion() {
    assert_eq!(hsv_to_rgb(0.0, 1.0, 1.0), (255, 0, 0));
    assert_eq!(hsv_to_rgb(120.0, 1.0, 1.0), (0, 255, 0));
    assert_eq!(hsv_to_rgb(240.0, 1.0, 1.0), (0, 0, 255));
    assert_eq!(hsv_to_rgb(60.0, 1.0, 1.0), (255, 255, 0));
    // s=0 → gray, v scales it.
    assert_eq!(hsv_to_rgb(0.0, 0.0, 1.0), (255, 255, 255));
    assert_eq!(hsv_to_rgb(0.0, 0.0, 0.0), (0, 0, 0));
    // Hue wraps.
    assert_eq!(hsv_to_rgb(480.0, 1.0, 1.0), hsv_to_rgb(120.0, 1.0, 1.0));
}

#[test]
fn gradients_cover_every_char() {
    let fade = gradient("abc", Color::BLACK, Color::WHITE);
    assert_eq!(fade.spans().len(), 3);
    assert_eq!(fade.visible_width(), 3);

    let single = gradient("x", Color::BLACK, Color::WHITE);
    assert_eq!(single.spans().len(), 1);
    assert_eq!(single.spans()[0].style().foreground(), Some(Color::BLACK));

    let empty = gradient("", Color::BLACK, Color::WHITE);
    assert_eq!(empty.spans().len(), 0);

    // Endpoints match the requested colors.
    let fade = gradient("ab", Color::RED, Color::BLUE);
    assert_eq!(
        fade.spans()[0].style().foreground().unwrap().rgb_value(),
        (205, 0, 0)
    );
    assert_eq!(
        fade.spans()[1].style().foreground().unwrap().rgb_value(),
        (0, 0, 238) // palette blue
    );
}

#[test]
fn gradient_bg_and_rainbow() {
    let bg = gradient_bg("hey", Color::BLACK, Color::WHITE);
    assert_eq!(bg.spans().len(), 3);
    assert!(bg.spans()[0].style().background().is_some());

    let arc = rainbow("12345");
    assert_eq!(arc.spans().len(), 5);
    // First hue is red (0°).
    assert_eq!(
        arc.spans()[0].style().foreground(),
        Some(Color::rgb(255, 0, 0))
    );
}
