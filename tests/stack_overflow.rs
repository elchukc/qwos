#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
use core::panic::PanicInfo;
use qwos::{exit_qemu, serial_print, serial_println, QemuExitCode};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[no_mangle]
pub extern "C" fn _start() -> ! {
  serial_print!("stack_overflow::stack_overflow...\t");

  qwos::gdt::init();
  init_test_idt();

  stack_overflow();

  panic!("Execution continued after stack overflow");
}

lazy_static! {
  static ref TEST_IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
      idt.double_fault
        .set_handler_fn(test_double_fault_handler)
        .set_stack_index(qwos::gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt
  };
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  // trigger a page fault by recursing endlessly
  stack_overflow();
  // prevent tail recursion optimizations
  volatile::Volatile::new(0).read();
}

pub fn init_test_idt() {
  TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  qwos::test_panic_handler(info)
}