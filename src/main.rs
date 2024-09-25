#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(qwos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use qwos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  qwos::init();

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[test_case]
fn test_println_simple() {
  println!("test_println_simple_output");
}

#[test_case]
fn test_println_many() {
  for _ in 0..200 {
    println!("test_println_many output");
  }
}