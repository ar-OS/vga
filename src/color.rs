/*
 * Represents a VGA's 16-color-modes.
 * Each enum is an hexadecimal representation of the given color.
 */
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    BrightBlue = 0x9,
    BrightGreen = 0xA,
    BrightCyan = 0xB,
    BrightRed = 0xC,
    BrightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

/*
 * Represents, as a byte, the background color (4 first bits) and the foreground
 * color (4 last bits) of a printable character.
 */
#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    /*
     * Returns a new ColorCode struct, according to a given foreground,
     * and a background color, both Color type.
     */
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

impl Default for ColorCode {
    /*
     * Returns a default ColorCode structure.
     * For the default one, a Green color foreground and a Black color
     * background has been choiced.
     */
    fn default() -> ColorCode {
        ColorCode::new(Color::Green, Color::Black)
    }
}
