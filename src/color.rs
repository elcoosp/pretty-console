//! Terminal colors: palette indices, 24-bit RGB, parsing, degradation and
//! allocation-free SGR emission.

use std::fmt;

use crate::choice::ColorLevel;

/// The ANSI reset sequence appended after styled text.
pub(crate) const RESET: &str = "\x1b[0m";

/// Default sRGB values of the 16 base ANSI palette entries, used for
/// luminance, contrast and degradation computations.
const ANSI16_RGB: [(u8, u8, u8); 16] = [
    (0, 0, 0),       // black
    (205, 0, 0),     // red
    (0, 205, 0),     // green
    (205, 205, 0),   // yellow
    (0, 0, 238),     // blue
    (205, 0, 205),   // magenta
    (0, 205, 205),   // cyan
    (229, 229, 229), // white
    (127, 127, 127), // bright black
    (255, 0, 0),     // bright red
    (0, 255, 0),     // bright green
    (255, 255, 0),   // bright yellow
    (92, 92, 255),   // bright blue
    (255, 0, 255),   // bright magenta
    (0, 255, 255),   // bright cyan
    (255, 255, 255), // bright white
];

/// Writes `value` as decimal digits without going through `format!`.
pub(crate) fn write_u8(out: &mut impl fmt::Write, value: u8) -> fmt::Result {
    let mut buf = [0u8; 3];
    let mut i = buf.len();
    let mut rest = value;
    loop {
        i -= 1;
        buf[i] = b'0' + (rest % 10);
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    // The slice is ASCII digits, so this conversion always succeeds.
    match std::str::from_utf8(&buf[i..]) {
        Ok(digits) => out.write_str(digits),
        Err(_) => Err(fmt::Error),
    }
}

/// Joins SGR parameters with `;` (a no-op before the first parameter).
#[cfg(not(feature = "no-color"))]
pub(crate) fn write_sep(out: &mut impl fmt::Write, first: &mut bool) -> fmt::Result {
    if *first {
        *first = false;
        Ok(())
    } else {
        out.write_str(";")
    }
}

/// An error returned when a color string cannot be parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseColorError {
    /// The string was empty or had an unexpected length (expected 3 or 6 hex
    /// digits, optionally prefixed with `#`).
    InvalidLength,
    /// The string contained a character that is not a hex digit.
    InvalidDigit(char),
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => f.write_str(
                "invalid color: expected 3 or 6 hex digits, optionally prefixed with '#'",
            ),
            Self::InvalidDigit(c) => write!(f, "invalid hex digit: {c:?}"),
        }
    }
}

impl std::error::Error for ParseColorError {}

/// A terminal color: a palette index or a 24-bit RGB value.
///
/// # Examples
///
/// ```
/// use pretty_console::Color;
///
/// let red = Color::RED;
/// let sky = Color::rgb(64, 164, 255);
/// let amber = Color::hex(0xFFBF00);
/// assert_eq!(amber, Color::RGB(255, 191, 0));
/// # let _ = (red, sky);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// A palette index into the 256-color table (0-255).
    Named(u8),
    /// A 24-bit RGB color.
    RGB(u8, u8, u8),
}

impl Color {
    /// Black (palette index 0).
    pub const BLACK: Color = Color::Named(0);
    /// Red (palette index 1).
    pub const RED: Color = Color::Named(1);
    /// Green (palette index 2).
    pub const GREEN: Color = Color::Named(2);
    /// Yellow (palette index 3).
    pub const YELLOW: Color = Color::Named(3);
    /// Blue (palette index 4).
    pub const BLUE: Color = Color::Named(4);
    /// Magenta (palette index 5).
    pub const MAGENTA: Color = Color::Named(5);
    /// Cyan (palette index 6).
    pub const CYAN: Color = Color::Named(6);
    /// White (palette index 7).
    pub const WHITE: Color = Color::Named(7);
    /// Bright black / gray (palette index 8).
    pub const BRIGHT_BLACK: Color = Color::Named(8);
    /// Bright red (palette index 9).
    pub const BRIGHT_RED: Color = Color::Named(9);
    /// Bright green (palette index 10).
    pub const BRIGHT_GREEN: Color = Color::Named(10);
    /// Bright yellow (palette index 11).
    pub const BRIGHT_YELLOW: Color = Color::Named(11);
    /// Bright blue (palette index 12).
    pub const BRIGHT_BLUE: Color = Color::Named(12);
    /// Bright magenta (palette index 13).
    pub const BRIGHT_MAGENTA: Color = Color::Named(13);
    /// Bright cyan (palette index 14).
    pub const BRIGHT_CYAN: Color = Color::Named(14);
    /// Bright white (palette index 15).
    pub const BRIGHT_WHITE: Color = Color::Named(15);

    /// Creates a color from a 256-color palette index.
    #[must_use]
    pub const fn indexed(index: u8) -> Self {
        Self::Named(index)
    }

    /// Creates a 24-bit RGB color.
    #[must_use]
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::RGB(red, green, blue)
    }

    /// Creates a 24-bit RGB color from a packed `0xRRGGBB` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Color;
    ///
    /// assert_eq!(Color::hex(0x00FF7F), Color::RGB(0, 255, 127));
    /// ```
    #[must_use]
    pub const fn hex(value: u32) -> Self {
        Self::RGB(
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        )
    }

    /// Parses a color from a hex string such as `"#7c4dff"`, `"7c4dff"` or
    /// the shorthand `"#f0a"`.
    ///
    /// # Errors
    ///
    /// Returns [`ParseColorError`] when the string is not 3 or 6 hex digits
    /// (with an optional leading `#`) or contains non-hex characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Color;
    ///
    /// assert_eq!(Color::from_hex_str("#7c4dff"), Ok(Color::rgb(124, 77, 255)));
    /// assert_eq!(Color::from_hex_str("f0a"), Ok(Color::rgb(255, 0, 170)));
    /// assert!(Color::from_hex_str("nope").is_err());
    /// ```
    pub fn from_hex_str(input: &str) -> Result<Self, ParseColorError> {
        let digits = input.strip_prefix('#').unwrap_or(input);
        let value = match digits.len() {
            3 => {
                let mut packed = 0u32;
                for c in digits.chars() {
                    let v = hex_digit(c).ok_or(ParseColorError::InvalidDigit(c))?;
                    // Expand "f0a" into "ff00aa".
                    packed = (packed << 8) | (v << 4) | v;
                }
                packed
            }
            6 => {
                let mut packed = 0u32;
                for c in digits.chars() {
                    let v = hex_digit(c).ok_or(ParseColorError::InvalidDigit(c))?;
                    packed = (packed << 4) | v;
                }
                packed
            }
            _ => return Err(ParseColorError::InvalidLength),
        };
        Ok(Self::hex(value))
    }

    /// Returns the SGR parameters that select this color as the foreground.
    #[must_use]
    pub fn fg_code(&self) -> String {
        let mut out = String::new();
        let _ = self.write_fg(&mut out);
        out
    }

    /// Returns the SGR parameters that select this color as the background.
    #[must_use]
    pub fn bg_code(&self) -> String {
        let mut out = String::new();
        let _ = self.write_bg(&mut out);
        out
    }

    /// Writes the SGR parameters selecting this color as the foreground.
    ///
    /// This is the allocation-free path used by the render machinery.
    pub(crate) fn write_fg(self, out: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Self::Named(index) => {
                out.write_str("38;5;")?;
                write_u8(out, index)
            }
            Self::RGB(r, g, b) => {
                out.write_str("38;2;")?;
                write_u8(out, r)?;
                out.write_str(";")?;
                write_u8(out, g)?;
                out.write_str(";")?;
                write_u8(out, b)
            }
        }
    }

    /// Writes the SGR parameters selecting this color as the background.
    pub(crate) fn write_bg(self, out: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Self::Named(index) => {
                out.write_str("48;5;")?;
                write_u8(out, index)
            }
            Self::RGB(r, g, b) => {
                out.write_str("48;2;")?;
                write_u8(out, r)?;
                out.write_str(";")?;
                write_u8(out, g)?;
                out.write_str(";")?;
                write_u8(out, b)
            }
        }
    }

    /// Returns the concrete sRGB triple this color displays as.
    ///
    /// Palette indices are resolved through the standard xterm palette.
    #[must_use]
    pub fn rgb_value(&self) -> (u8, u8, u8) {
        match *self {
            Self::RGB(r, g, b) => (r, g, b),
            Self::Named(index) => palette_rgb(index),
        }
    }

    /// Relative luminance following the WCAG sRGB definition (0.0 = black,
    /// 1.0 = white).
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Color;
    ///
    /// assert_eq!(Color::BLACK.luminance(), 0.0);
    /// assert!(Color::BRIGHT_WHITE.luminance() > 0.99);
    /// ```
    #[must_use]
    pub fn luminance(&self) -> f32 {
        fn channel(c: u8) -> f32 {
            let c = f32::from(c) / 255.0;
            if c <= 0.040_45 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        let (r, g, b) = self.rgb_value();
        0.212_6 * channel(r) + 0.715_2 * channel(g) + 0.072_2 * channel(b)
    }

    /// Returns black or white, whichever contrasts more with this color used
    /// as a background (WCAG 4.5:1 crossover).
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Color;
    ///
    /// assert_eq!(Color::BLACK.readable_on(), Color::WHITE);
    /// assert_eq!(Color::YELLOW.readable_on(), Color::BLACK);
    /// ```
    #[must_use]
    pub fn readable_on(&self) -> Color {
        // White text beats black text when (L + 0.05) <= sqrt(0.05 * 1.05).
        if self.luminance() <= 0.178_325_48 {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }

    /// Linear interpolation toward `other` by `t` in `[0, 1]`.
    ///
    /// Colors are converted to RGB first, so palette indices interpolate in
    /// RGB space too.
    ///
    /// # Examples
    ///
    /// ```
    /// use pretty_console::Color;
    ///
    /// let mid = Color::lerp(Color::BLACK, Color::BRIGHT_WHITE, 0.5);
    /// assert_eq!(mid, Color::RGB(128, 128, 128));
    /// ```
    #[must_use]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        if t <= 0.0 {
            return self;
        }
        if t >= 1.0 {
            return other;
        }
        let (r1, g1, b1) = self.rgb_value();
        let (r2, g2, b2) = other.rgb_value();
        Self::RGB(mix(r1, r2, t), mix(g1, g2, t), mix(b1, b2, t))
    }

    /// Nearest 256-palette entry to this color.
    ///
    /// Palette indices are returned unchanged; RGB colors are mapped to the
    /// closest of the 6x6x6 color cube and the 24-step grayscale ramp.
    #[must_use]
    pub fn to_256(self) -> Self {
        let index = match self {
            Self::Named(index) => index,
            Self::RGB(r, g, b) => rgb_to_256(r, g, b),
        };
        Self::Named(index)
    }

    /// Nearest of the 16 base ANSI colors to this color.
    #[must_use]
    pub fn to_16(self) -> Self {
        let (r, g, b) = self.rgb_value();
        let mut best = 0usize;
        let mut best_dist = u32::MAX;
        for (i, &(pr, pg, pb)) in ANSI16_RGB.iter().enumerate() {
            let dr = i32::from(r) - i32::from(pr);
            let dg = i32::from(g) - i32::from(pg);
            let db = i32::from(b) - i32::from(pb);
            let dist = (dr * dr + dg * dg + db * db) as u32;
            if dist < best_dist {
                best_dist = dist;
                best = i;
            }
        }
        Self::Named(best as u8)
    }

    /// Reduces this color to the richest representation `level` supports.
    #[must_use]
    pub fn degrade_to(self, level: ColorLevel) -> Self {
        match level {
            ColorLevel::TrueColor => self,
            ColorLevel::Ansi256 => match self {
                Self::Named(_) => self,
                Self::RGB(..) => self.to_256(),
            },
            ColorLevel::Ansi16 => match self {
                Self::Named(index) if index <= 15 => self,
                _ => self.to_16(),
            },
        }
    }

    /// The minimum [`ColorLevel`] needed to display this color exactly.
    #[must_use]
    pub fn level(self) -> ColorLevel {
        match self {
            Self::Named(index) if index <= 15 => ColorLevel::Ansi16,
            Self::Named(_) => ColorLevel::Ansi256,
            Self::RGB(..) => ColorLevel::TrueColor,
        }
    }
}

impl From<u8> for Color {
    fn from(index: u8) -> Self {
        Self::Named(index)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::RGB(r, g, b)
    }
}

impl std::str::FromStr for Color {
    type Err = ParseColorError;

    /// Parses hex color strings; see [`Color::from_hex_str`].
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_hex_str(input)
    }
}

/// Resolves a 256-palette index to its standard xterm RGB value.
#[must_use]
pub fn palette_rgb(index: u8) -> (u8, u8, u8) {
    match index {
        0..=15 => ANSI16_RGB[index as usize],
        16..=231 => {
            let i = u32::from(index) - 16;
            let levels = [0u8, 95, 135, 175, 215, 255];
            let r = levels[(i / 36) as usize];
            let g = levels[((i / 6) % 6) as usize];
            let b = levels[(i % 6) as usize];
            (r, g, b)
        }
        232..=255 => {
            let gray = 8 + 10 * (u32::from(index) - 232);
            (gray as u8, gray as u8, gray as u8)
        }
    }
}

/// Nearest 256-palette index for an RGB triple.
#[must_use]
fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
    const CUBE: [u8; 6] = [0, 95, 135, 175, 215, 255];

    let nearest_level = |c: u8| -> (usize, u32) {
        let mut best = 0usize;
        let mut best_dist = u32::MAX;
        for (i, &level) in CUBE.iter().enumerate() {
            let d = i32::from(c) - i32::from(level);
            let dist = (d * d) as u32;
            if dist < best_dist {
                best_dist = dist;
                best = i;
            }
        }
        (best, best_dist)
    };

    let (ri, rd) = nearest_level(r);
    let (gi, gd) = nearest_level(g);
    let (bi, bd) = nearest_level(b);
    let cube_dist = rd + gd + bd;
    let cube_index = 16 + 36 * ri + 6 * gi + bi;

    // Grayscale ramp: 232..=255 map to 8, 18, ..., 238.
    let gray_avg = (u32::from(r) + u32::from(g) + u32::from(b)) / 3;
    let gray_index = 232 + (gray_avg.saturating_sub(8) / 10).min(23);
    let gray_val: u8 = (8 + 10 * (gray_index - 232)) as u8;
    let gd2 = |c: u8| {
        let d = i32::from(c) - i32::from(gray_val);
        (d * d) as u32
    };
    let gray_dist = gd2(r) + gd2(g) + gd2(b);

    if gray_dist < cube_dist {
        gray_index as u8
    } else {
        cube_index as u8
    }
}

fn hex_digit(c: char) -> Option<u32> {
    c.to_digit(16)
}

fn mix(a: u8, b: u8, t: f32) -> u8 {
    let value = f32::from(a) * (1.0 - t) + f32::from(b) * t;
    value.round().clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminance_of_palette_entries() {
        assert_eq!(Color::BLACK.luminance(), 0.0);
        assert!(Color::BRIGHT_WHITE.luminance() > 0.99);
        // Base white (palette 7) is a mid-bright gray.
        assert!(Color::WHITE.luminance() > 0.7);
        assert!(Color::WHITE.luminance() < 0.9);
    }
}
