//! exe.rs --- shellcode executor

//! don't forget to enable the 'exe' feature and supply your own
//! code.bin in the crate root.
use std::mem;

// do this trick because otherwise only the reference is in the .text section
const SHELLCODE_BYTES: &[u8] = include_bytes!("../code.bin");
const SHELLCODE_LENGTH: usize = SHELLCODE_BYTES.len();

#[no_mangle]
#[link_section = ".text"]
static SHELLCODE: [u8; SHELLCODE_LENGTH] = *include_bytes!("../code.bin");

pub fn static_exe() {
    let exec_shellcode: extern "C" fn() -> ! =
    unsafe { mem::transmute(&SHELLCODE as *const _ as *const ()) };
    exec_shellcode();
}

pub fn dyn_exe(code: &[u8]) {
  let exec_shellcode: extern "C" fn() -> ! =
    unsafe { mem::transmute(&code as *const _ as *const ()) };
  exec_shellcode();
}
