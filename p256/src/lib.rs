#![no_std]
use core::slice::from_raw_parts;
use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};

mod syscalls;

#[panic_handler]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    syscalls::debug_slice(b"panic\0");
    syscalls::exit(-1)
}

#[no_mangle]
pub extern "C" fn verify111(
    public_key: *const u8,
    public_key_length: u32,
    message: *const u8,
    message_length: u32,
    signature: *const u8,
    signature_length: u32,
) -> i32 {
    let public_key = unsafe { from_raw_parts(public_key, public_key_length as usize) };
    let message = unsafe { from_raw_parts(message, message_length as usize) };
    let signature = unsafe { from_raw_parts(signature, signature_length as usize) };

    let signature = match Signature::from_slice(signature) {
        Ok(signature) => signature,
        Err(_) => {
            syscalls::debug_slice(b"Signature decoding error\0");
            return 1;
        }
    };

    let pk = match VerifyingKey::from_sec1_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => {
            syscalls::debug_slice(b"Verifying key decoding error\0");
            return 2;
        }
    };

    if let Err(_) = pk.verify(message, &signature) {
        syscalls::debug_slice(b"Verification error\0");
        return 3;
    }

    0
}
