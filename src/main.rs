#![no_main]
#![no_std]

use core::{arch::asm, panic::PanicInfo};
use cortex_m::asm;
use cortex_m::interrupt;
use cortex_m::Peripherals;
use cortex_m_semihosting::hprintln;
use cortex_m::peripheral::syst::SystClkSource;

use cortex_m::peripheral::NVIC;

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}


#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 48] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },

    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: TIM4 },
    Vector {  reserved: 0},
    Vector {  reserved: 0 },
    Vector {  reserved: 0},
    //Vector { handler: TIM4 },
];

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    //let _x = 42;

    //this is just for unmasking an interrupt
    asm!("cpsie i");
    const TIM4_IRQ: u32 = 30;
  
    const NVIC_ISER0: *mut u32 = 0xE000_E100 as *mut u32;
    core::ptr::write_volatile(NVIC_ISER0, 1 << TIM4_IRQ);

    // Pend TIM4 IRQ
    const NVIC_ISPR0: *mut u32 = 0xE000_E200 as *mut u32;
    core::ptr::write_volatile(NVIC_ISPR0, 1 << TIM4_IRQ);


    // //let core = Peripherals::steal();
    // NVIC::unmask(30);

    asm!("svc 0");
    // can't return so we go into an infinite loop here
    loop {
       // hprintln!("test..");
    }

}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}


#[no_mangle]
pub extern "C" fn NMI() {
    
}

#[no_mangle]
pub extern "C" fn HardFault() {
    //hprintln!("I faulted");
   
}

#[no_mangle]
pub extern "C" fn MemManage() {
  
}

#[no_mangle]
pub extern "C" fn BusFault() {
   
}

#[no_mangle]
pub extern "C" fn UsageFault() {
}
#[no_mangle]
pub extern "C" fn SVCall() {
    hprintln!("Test..");
}

#[no_mangle]
pub extern "C" fn PendSV() {
    //hprintln!("I pended SV");
}

#[no_mangle]
pub extern "C" fn SysTick() {
}


#[no_mangle]
pub  extern "C" fn TIM4(){
    hprintln!("test4..").unwrap();
}