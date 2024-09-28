use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{gdt, println, print};
use pic8259::ChainedPics;
use spin;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
  unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }
);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
  Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
  fn as_u8(self) -> u8 {
    self as u8
  }

  fn as_usize(self) -> usize {
    usize::from(self.as_u8())
  }
}

lazy_static!{
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
      idt.double_fault.set_handler_fn(double_fault_handler)
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    // NOTE the example had this as .as_usize()
    idt[InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);

    idt
  };
}

pub fn init_idt() {
  IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
  println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
  println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
  panic!("Should never return from double fault interrupt");
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
  print!(".");
}

#[test_case]
fn test_breakpoint_exception() {
  // invoke a breakpoint exception
  x86_64::instructions::interrupts::int3();
}