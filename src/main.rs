//disabling standard lib
#![no_std]

//imports
use core::panic::PanicInfo;

// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) ->!{
    loop{}
}
fn main() {
}
