#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(os_in_rust::test_runner)]

use core::panic::PanicInfo;
use os_in_rust::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output")
}