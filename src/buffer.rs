use color::ColorCode;
use core::ptr::Unique;
use core::fmt::{self, Write};
use spin::Mutex;
use volatile::Volatile;

/// Memory address of the VGA buffer
const VGA_BUFFER_ADDRESS: usize = 0xb8000;

/// Length of a line
const BUFFER_LENGTH: usize = 80;

/// Number of lines
const BUFFER_HEIGHT: usize = 24;

///Each printable character contains his own color, and the byte to display
type Char = (u8, ColorCode);

/// Represents a printable buffer
///
/// This buffer will contain printable characters, displayable in the ar-OS console.
/// I used volatile writes for the VGA buffer in order to avoid optimizations from the compiler,
/// and to display easily (without troubles) characters on the screen (you can check
/// [this blog post](https://os.phil-opp.com/printing-to-screen/#volatile) if you want more informations).
///
/// For volatile reads/writes, I used the [Volatile](https://github.com/embed-rs/volatile) crate.
pub struct Buffer {
    content: [[Volatile<Char>; BUFFER_LENGTH]; BUFFER_HEIGHT],
}

/// Contains necessary informations to write text on screen:
/// * column_position: the position of the cursor for the last printable line;
/// * color_code: the color code for the current printable character;
/// * buffer: previous, current and future text container (to be printed).
pub struct Writer {
    buffer: Unique<Buffer>,
    color_code: ColorCode,
    column_position: usize,
    row_position: usize,
}

impl Writer {
    /// Implement a new Writer data structure
    pub fn new(
        buffer: Unique<Buffer>,
        color_code: ColorCode,
        column_position: usize,
        row_position: usize,
    ) -> Writer {
        Writer {
            buffer,
            color_code,
            column_position,
            row_position,
        }
    }

    /// Clear the screen entirely
    pub fn clear(&mut self) {
        let color_code = self.color_code;
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_LENGTH {
                self.buffer().content[row][col].write((b' ', color_code));
            }
        }
    }

    /// Add an empty new line in the buffer
    ///
    /// Using this method, the `row_position` will move to the next one if
    /// the current row position is lower than `BUFFER_HEIGHT`.
    /// Also, we move the `column_position` field to 0.
    pub fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT {
            self.row_position += 1;
        }
        self.column_position = 0;
    }

    /// Write a single byte into the current buffer
    pub fn write_byte(&mut self, byte: u8) {
        let row = self.row_position;
        let col = self.column_position;
        let color_code = self.color_code;
        match char::from(byte) {
            '\n' => {
                self.new_line();
            }
            _ => {
                if self.column_position >= BUFFER_LENGTH {
                    self.new_line();
                }
                // Clone the column_position fields
                // Change the content buffer
                self.buffer().content[row][col].write((byte, color_code));
                self.column_position += 1;
            }
        }
    }

    /// Returns a mutable reference to the current internal buffer data structure
    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }
}

impl fmt::Write for Writer {
    /// Write a given string to the current buffer
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

/*
 * Static API to write something in the console.ColorCode
 * Usage of lazy_static here to trick to compiler in
 * initializing a Color value in runtime... instead of compile time ;)
 */
lazy_static! {
    pub static ref BUF_WRITER: Writer = Writer {
        buffer: unsafe { Unique::new_unchecked(VGA_BUFFER_ADDRESS as *mut _) },
        color_code: ColorCode::default(),
        column_position: 0,
        row_position: 0,
    };
}

/// Write a given string to the given Writer structure
macro_rules! echo {
    ($writer: expr, $($arg:tt)*) => ({
        $writer.write_fmt(format_args!($($arg)*)).unwrap();
        // $writer.new_line();
    });
}

/// Print a message to the screen
pub fn print_message() {
    let mut stdout = Writer::new(
        unsafe { Unique::new_unchecked(VGA_BUFFER_ADDRESS as *mut _) },
        ColorCode::default(),
        0,
        0,
    );
    stdout.clear();
    for i in 0..100 {
        echo!(stdout, "Hello {}\n", "world!");
    }
}
