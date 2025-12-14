use crate::gdt;
use crate::hlt_loop;
use lazy_static::lazy_static;
use log::error;
use log::info;
use log::trace;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        idt.page_fault.set_handler_fn(page_fault_handler);

        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt
            .set_handler_fn(non_maskable_interrupt_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available
            .set_handler_fn(device_not_available_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present
            .set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault
            .set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt.x87_floating_point
            .set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point
            .set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.cp_protection_exception
            .set_handler_fn(cp_protection_exception_handler);
        idt.hv_injection_exception
            .set_handler_fn(hv_injection_exception_handler);
        idt.vmm_communication_exception
            .set_handler_fn(vmm_communication_exception_handler);
        idt.security_exception
            .set_handler_fn(security_exception_handler);

        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    trace!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: divide_error\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: debug\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: non_maskable_interrupt\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: overflow\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: bound_range_exceeded\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: invalid_opcode\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: device_not_available\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, code: u64) {
    trace!("INTERRUPT: invalid_tss\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame, code: u64) {
    trace!("INTERRUPT: segment_not_present\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, code: u64) {
    trace!("INTERRUPT: stack_segment_fault\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    code: u64,
) {
    trace!(
        "INTERRUPT: general_protection_fault {}\n{:#?}",
        code, stack_frame
    );
}

extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: x87_floating_point\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame, code: u64) {
    trace!("INTERRUPT: alignment_check\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("INTERRUPT: machine_check\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: simd_floating_point\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: virtualization\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn cp_protection_exception_handler(
    stack_frame: InterruptStackFrame,
    code: u64,
) {
    trace!("INTERRUPT: cp_protection_exception\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn hv_injection_exception_handler(stack_frame: InterruptStackFrame) {
    trace!("INTERRUPT: hv_injection_exception\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn vmm_communication_exception_handler(
    stack_frame: InterruptStackFrame,
    code: u64,
) {
    trace!("INTERRUPT: vmm_communication_exception\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn security_exception_handler(stack_frame: InterruptStackFrame, code: u64) {
    trace!("INTERRUPT: security_exception\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    info!(".");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    info!("keyboard_interrupt_handler");
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    info!("keyboard_interrupt_handler: add new scancode");
    crate::task::keyboard::add_scancode(scancode); // new

    info!("keyboard_interrupt_handler: notify end of interrupt");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    info!("page_fault_handler");
    use x86_64::registers::control::Cr2;

    error!("EXCEPTION: PAGE FAULT");
    error!("Accessed Address: {:?}", Cr2::read());
    error!("Error Code: {:?}", error_code);
    error!("{:#?}", stack_frame);
    hlt_loop();
}

pub fn init_idt() {
    IDT.load();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
