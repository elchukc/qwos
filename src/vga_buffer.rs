use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use font_constants::BACKUP_CHAR;
use bootloader_api::info::{FrameBufferInfo,  PixelFormat};
use noto_sans_mono_bitmap::{get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar};

/// Additional vertical space between lines
const LINE_SPACING: usize = 2;
/// Additional horizontal space between characters.
const LETTER_SPACING: usize = 0;
/// Padding from the border. Prevent that font is too close to border.
const BORDER_PADDING: usize = 1;

/// Constants for the usage of the [`noto_sans_mono_bitmap`] crate.
mod font_constants {
  use super::*;
  /// Height of each char raster. The font size is ~0.84% of this. Thus, this is the line height that
  /// enables multiple characters to be side-by-side and appear optically in one line in a natural way.
  pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;
  /// The width of each single symbol of the mono space font.
  pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);
  /// Backup character if a desired symbol is not available by the font.
  /// The '�' character requires the feature "unicode-specials".
  pub const BACKUP_CHAR: char = '�';
  pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
}

fn get_char_raster(c: char)  -> RasterizedChar {
  fn get(c: char) -> Option<RasterizedChar> {
    get_raster(c, font_constants::FONT_WEIGHT, font_constants::CHAR_RASTER_HEIGHT)
  }
  get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
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
pub fn _print(args: fmt::Arguments) {
  use core::fmt::Write;
  use x86_64::instructions::interrupts;

  interrupts::without_interrupts(|| {
    WRITER.lock().write_fmt(args).unwrap();
  });
}

struct FrameBufferWriter {
  framebuffer: &'static mut [u8],
  info: FrameBufferInfo,
  x_pos: usize,
  y_pos: usize,
}

impl FrameBufferWriter {
  /// Creates a new logger that uses the given framebuffer
  pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
    let mut logger = Self {
      framebuffer,
      info,
      x_pos: 0,
      y_pos: 0,
    };
    logger.clear();
    logger
  }

  /// Writes a single char to the framebuffer. Takes care of special control characters, such as
  /// newlines and carriage returns.
  fn write_char(&mut self, c: char) {
    match c {
      '\n' => self.new_line(),
      '\r' => self.carriage_return(),
      c => {
        let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
        if new_xpos >= self.width() {
          self.new_line()
        }
        let new_ypos = self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
        if new_ypos >= self.height() {
          self.clear();
        }
        self.write_rendered_char(get_char_raster(c));
      }
    }
  }

  /// Prints a rendered char into the framebuffer.
  /// Updates `self.x_pos`.
  fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
    for (y, row) in rendered_char.raster().iter().enumerate() {
      for (x, byte) in row.iter().enumerate() {
        self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
      }
    }
    self.x_pos += rendered_char.width() + LETTER_SPACING;
  }

  // See bootloader's framebuffer example if you want this
  // fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {...}

  fn width(&self) -> usize {
    self.info.width
  }

  fn height(&self) -> usize {
    self.info.height
  }

  fn new_line(&mut self) {
    self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
    self.carriage_return(&mut self) {
      self.x_pos = BORDER_PADDING;
    }
  }

  pub fn clear(&mut self) {
    self.x_pos = BORDER_PADDING;
    self.y_pos = BORDER_PADDING;
    self.framebuffer.fill(0);
  }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferInfoWriter {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    for c in s.chars() {
      self.write_char(c);
    }
    Ok(())
  }
}
// pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
//   let mut logger = Self {
//     framebuffer,
//     info,
//     x_pos: 0,
//     y_pos: 0,
//   };
//   logger.clear();
//   logger
// }

lazy_static! {
  pub static ref WRITER: Mutex<FrameBufferWriter> = Mutex::new(FrameBufferWriter {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
  pub static ref FRAMEBUFFER: Mutex<FrameBufferWriter> = Mutex::new(FrameBufferWriter::new(
    unsafe {&mut *(0xb8000 as mut* FrameBuffer)},
    
  ));
}

#[test_case]
fn test_println_output() {
  use core::fmt::Write;
  use x86_64::instructions::interrupts;

  let s = "Some test string that fits on a single line.";
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    writeln!(writer, "\n{}", s).expect("writeln failed");
    for (i,c) in s.chars().enumerate() {
      let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
      assert_eq!(char::from(screen_char.ascii_character), c);
    }
  });
}