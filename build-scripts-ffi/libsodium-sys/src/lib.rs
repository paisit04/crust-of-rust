#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Sodium;

impl Sodium {
    fn new() -> Result<Self, ()> {
        if unsafe { ffi::sodium_init() } < 0 {
            Err(())
        } else {
            Ok(Self)
        }
    }

    pub fn crypto_generichash<'a>(
        &self,
        input: &[u8],
        key: Option<&[u8]>,
        out: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut [u8], ()> {
        assert!(out.len() >= usize::try_from(ffi::crypto_generichash_BYTES_MIN).unwrap());
        assert!(out.len() <= usize::try_from(ffi::crypto_generichash_BYTES_MAX).unwrap());

        if let Some(key) = key {
            assert!(key.len() >= usize::try_from(ffi::crypto_generichash_KEYBYTES_MIN).unwrap());
            assert!(key.len() <= usize::try_from(ffi::crypto_generichash_KEYBYTES_MAX).unwrap());
        }

        let (key, keylen) = if let Some(key) = key {
            (key.as_ptr(), key.len())
        } else {
            (std::ptr::null(), 0)
        };

        // SAFETY: We've checked the requirements of the function (MIN/MAX), and the presence of
        // &self means that init has been called.
        let res = unsafe {
            ffi::crypto_generichash(
                MaybeUninit::slice_as_mut_ptr(out),
                out.len(),
                input.as_ptr(),
                input.len() as u64,
                key,
                keylen,
            )
        };

        if res < 0 {
            return Err(());
        }

        // SAFETY: crypto_generichash writes to (and thus initializes) all the bytes of out
        Ok(MaybeUninit::slice_assume_init_mut(out))
    }
}

use std::mem::MaybeUninit;

pub use ffi::sodium_init;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Sodium::new().unwrap();
    }
}
