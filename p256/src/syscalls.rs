use core::ffi::CStr;

unsafe fn syscall(mut a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a7: u64) -> u64 {
    core::arch::asm!(
      "ecall",
      inout("a0") a0,
      in("a1") a1,
      in("a2") a2,
      in("a3") a3,
      in("a4") a4,
      in("a5") a5,
      in("a7") a7
    );
    a0
}

const SYS_EXIT: u64 = 93;
const SYS_DEBUG: u64 = 2177;

pub fn exit(code: i8) -> ! {
    unsafe { syscall(code as u64, 0, 0, 0, 0, 0, SYS_EXIT) };
    loop {}
}

pub fn debug(message: &CStr) {
    unsafe { syscall(message.as_ptr() as u64, 0, 0, 0, 0, 0, SYS_DEBUG) };
}

pub fn debug_slice(message: &[u8]) {
    let message = unsafe { CStr::from_ptr(message.as_ptr().cast()) };
    debug(&message)
}
