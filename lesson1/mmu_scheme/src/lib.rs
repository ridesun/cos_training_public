#![no_std]

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;
const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;

#[cfg(feature = "sv39")]
mod sv39;
#[cfg(feature = "sv39")]
pub use sv39::*;
#[cfg(feature = "sv48")]
mod sv48;
#[cfg(feature = "sv48")]
pub use sv48::*;
