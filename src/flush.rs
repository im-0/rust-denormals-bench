// SPDX-License-Identifier: Apache-2.0

// Copied from from https://github.com/dimforge/rapier

macro_rules! enable_flush_to_zero(
    () => {
        let _flush_to_zero = crate::flush::FlushToZeroDenormalsAreZeroFlags::flush_denormal_to_zero();
    }
);
pub(crate) use enable_flush_to_zero;

// This is an RAII structure that enables flushing denormal numbers
// to zero, and automatically resetting previous flags once it is dropped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FlushToZeroDenormalsAreZeroFlags {
    #[cfg(any(not(any(
        target_arch = "aarch64",
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse"
        )
    ))))]
    original_flags: (),

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse"
    ))]
    original_flags: u32,

    #[cfg(all(target_arch = "aarch64"))]
    original_flags: u64,
}

// Flush denormals & underflows to zero as this as a significant impact on the solver's performances.
impl FlushToZeroDenormalsAreZeroFlags {
    #[cfg(any(not(any(
        target_arch = "aarch64",
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse"
        )
    ))))]
    pub fn flush_denormal_to_zero() -> Self {
        Self { original_flags: () }
    }

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse"
    ))]
    #[allow(deprecated)] // will address that later.
    pub fn flush_denormal_to_zero() -> Self {
        unsafe {
            #[cfg(target_arch = "x86")]
            use std::arch::x86::{_mm_getcsr, _mm_setcsr, _MM_FLUSH_ZERO_ON};
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::{_mm_getcsr, _mm_setcsr, _MM_FLUSH_ZERO_ON};

            // To enable this we need to set the bit 15 (given by _MM_FLUSH_ZERO_ON) and the bit 6 (for denormals-are-zero).
            // See https://software.intel.com/content/www/us/en/develop/articles/x87-and-sse-floating-point-assists-in-ia-32-flush-to-zero-ftz-and-denormals-are-zero-daz.html
            let original_flags = _mm_getcsr();
            _mm_setcsr(original_flags | _MM_FLUSH_ZERO_ON | (1 << 6));
            Self { original_flags }
        }
    }

    #[cfg(all(target_arch = "aarch64"))]
    pub fn flush_denormal_to_zero() -> Self {
        let mut original_flags: u64;
        unsafe {
            std::arch::asm!("mrs {}, fpcr", out(reg) original_flags);
            // This sets following bits of FPCR (Floating-point Control Register):
            //     FZ, bit 24 - Flushing denormalized numbers to zero
            //     FZ16, bit 19 - Enable flushing for half-precision (f16) numbers
            // See https://developer.arm.com/documentation/ddi0601/2025-06/AArch64-Registers/FPCR--Floating-point-Control-Register
            std::arch::asm!("msr fpcr, {}", in(reg) original_flags | (1 << 24) | (1 << 19));
        }
        Self { original_flags }
    }
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse"
))]
impl Drop for FlushToZeroDenormalsAreZeroFlags {
    #[allow(deprecated)] // will address that later.
    fn drop(&mut self) {
        #[cfg(target_arch = "x86")]
        unsafe {
            std::arch::x86::_mm_setcsr(self.original_flags)
        }
        #[cfg(target_arch = "x86_64")]
        unsafe {
            std::arch::x86_64::_mm_setcsr(self.original_flags)
        }
    }
}

#[cfg(all(target_arch = "aarch64"))]
impl Drop for FlushToZeroDenormalsAreZeroFlags {
    fn drop(&mut self) {
        unsafe { std::arch::asm!("msr fpcr, {}", in(reg) self.original_flags) }
    }
}
