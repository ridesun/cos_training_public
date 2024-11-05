use crate::PHYS_VIRT_OFFSET;
use core::ptr::addr_of;
use riscv::register::satp;
use riscv::register::satp::Mode;

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_SV48: [[u64; 512]; 3] = [[0; 512]; 3];

pub unsafe fn pre_mmu() {
    let page_table_root = addr_of!(BOOT_PT_SV48[1]) as u64;
    let page_table_id = page_table_root >> 12;
    BOOT_PT_SV48[0][0] = (page_table_id << 10) | 0x01;
    let page_table_root = addr_of!(BOOT_PT_SV48[2]) as u64;
    let page_table_id = page_table_root >> 12;
    BOOT_PT_SV48[0][0x1ff] = (page_table_id << 10) | 0x01;
    BOOT_PT_SV48[1][2] = (0x80000 << 10) | 0xef;
    BOOT_PT_SV48[2][0x102] = (0x80000 << 10) | 0xef;
    BOOT_PT_SV48[2][0x1ff] = (0x80000 << 10) | 0xef;
}

pub unsafe fn enable_mmu() {
    let page_table_root = BOOT_PT_SV48[0].as_ptr() as usize;
    satp::set(Mode::Sv48, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

pub unsafe fn post_mmu() {
    core::arch::asm!(
    "
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0
        add     ra, ra, t0
        ret     ",
    phys_virt_offset = const PHYS_VIRT_OFFSET,
    );
}
