use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub const DEFAULT_FOREGROUND_COLOR: Color = Color::White;
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::Black;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[macro_export]
macro_rules! color_code {
    () => {
        $crate::vga_buffer::ColorCode::new(
            $crate::vga_buffer::DEFAULT_FOREGROUND_COLOR,
            $crate::vga_buffer::DEFAULT_BACKGROUND_COLOR,
        )
    };
    ($fg:expr) => {
        $crate::vga_buffer::ColorCode::new($fg, $crate::vga_buffer::DEFAULT_BACKGROUND_COLOR)
    };
    ($fg:expr, $bg:expr) => {
        $crate::vga_buffer::ColorCode::new($fg, $bg)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        s.bytes().for_each(|byte| match byte {
            0x20..=0x7e | b'\n' => self.write_byte(byte),
            _ => self.write_byte(0xfe),
        });
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: color_code!(),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER
            .lock()
            .write_fmt(args)
            .expect("Printing to vga failed");
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _colored_print(color_code: ColorCode, args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        let original_color_code = writer.color_code;
        writer.color_code = color_code;
        writer.write_fmt(args).expect("Printing to vga failed");
        writer.color_code = original_color_code;
        drop(writer);
    });
}

#[macro_export]
macro_rules! colored_print {
    ($color_code:expr, $($arg:tt)*) => ($crate::vga_buffer::_colored_print($color_code, format_args!($($arg)*)));
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        let s = "Some test string that fits on a single line";
        println!("\n{}", s);
        s.chars().enumerate().for_each(|(i, c)| {
            assert_eq!(
                char::from(
                    WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i]
                        .read()
                        .ascii_character
                ),
                c
            );
        });
    });
}

#[test_case]
fn test_println_text_wrap() {
    use x86_64::instructions::interrupts;

    let s = "Some long line with many characters. Some long line with many characters. Some long line with many characters.";
    let num_lines = ((s.len() - 1) / BUFFER_WIDTH) as usize + 1;
    let start_line = BUFFER_HEIGHT - 1 - num_lines;
    interrupts::without_interrupts(|| {
        println!("\n{}", s);
        s.chars().enumerate().for_each(|(i, c)| {
            assert_eq!(
                char::from(
                    WRITER.lock().buffer.chars[start_line + (i / BUFFER_WIDTH)][i % BUFFER_WIDTH]
                        .read()
                        .ascii_character
                ),
                c
            );
        });
    });
}

#[test_case]
fn test_colored_print_output() {
    use x86_64::instructions::interrupts;

    let s = "Some colorful string";
    let color_code = color_code!(Color::Green, Color::Yellow);

    interrupts::without_interrupts(|| {
        println!();
        colored_print!(color_code, "{}", s);
        s.chars().enumerate().for_each(|(i, c)| {
            let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 1][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
            assert_eq!(screen_char.color_code, color_code)
        });
    });
}
