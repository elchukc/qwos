#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  use core::fmt::Write;
  vga_buffer::WRITER.lock().write_str("Hello again").unwrap();

  write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
  // vga_buffer::WRITER.lock().write_byte(b'H');
  // writer.write_string("ello ");
  // writer.write_string("Wörld!");
  // write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();

  loop {}
}