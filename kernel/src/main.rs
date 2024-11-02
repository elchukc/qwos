#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(qwos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader_api::entry_point;
use qwos::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
  println!("Hello World{}", "!");

  // read env variables that were set in build script
  let bios_path = env!("BIOS_PATH");

  qwos::init();

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  qwos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  qwos::hlt_loop();
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