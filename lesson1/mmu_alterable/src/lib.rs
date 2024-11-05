#![no_std]

use riscv::register::satp;
use riscv::register::satp::Mode;

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;

#[cfg(feature = "enable")]
const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;

#[cfg(feature = "enable")]
#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_SV39: [u64; 512] = [0; 512];
pub unsafe fn pre_mmu() {
    #[cfg(feature = "enable")]
    {
        BOOT_PT_SV39[2] = (0x80000 << 10) | 0xef;
        BOOT_PT_SV39[0x102] = (0x80000 << 10) | 0xef;
        BOOT_PT_SV39[0x1ff] = (0x80000 << 10) | 0xef;
    }
}

pub unsafe fn enable_mmu() {
    #[cfg(feature = "enable")]
    {
        let page_table_root = BOOT_PT_SV39.as_ptr() as usize;
        satp::set(Mode::Sv39, 0, page_table_root >> 12);
        riscv::asm::sfence_vma_all();
    }
}

pub unsafe fn post_mmu() {
    #[cfg(feature = "enable")]
    {
        core::arch::asm!(
        "
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0
        add     ra, ra, t0
        ret     ",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
        );
    }
}
