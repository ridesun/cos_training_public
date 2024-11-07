#![no_std]

use core::ptr::addr_of;
use riscv::register::satp;

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;
const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_ARRAY: [[u64; 512]; 3] = [[0; 512]; 3];

#[cfg(feature = "sv39")]
const MMU_LEVELS: usize = 3;

#[cfg(feature = "sv48")]
const MMU_LEVELS: usize = 4;


// 计算页表索引
// (MMU_LEVELS - level) 计算当前层级与顶级页表之间的距离
// 每级页表索引的位数为9，12减去3位PTE标志位
// +3是为了对齐到9位的边界
// 页大小2^12，PTE占2^3，每级页表有512条，511=0x1ff
fn boot_array_index(addr:usize,level:usize)->usize{
    (addr>>((MMU_LEVELS-level)*9+3))&0x1ff
}

// 对齐到PPN[0]
// PTEFlags V=1<<0
#[cfg(feature = "sv48")]
fn page_table_node(page_table_root:u64)->u64{
    ((page_table_root>>12)<<10)|(1<<0)
}

#[cfg(feature = "sv39")]
pub unsafe fn pre_mmu() {
    let index = boot_array_index(0x8000_0000, 0);
    BOOT_PT_ARRAY[0][index] = (0x80000 << 10) | 0xef;

    let index = boot_array_index(0xffff_ffc0_8000_0000, 0);
    BOOT_PT_ARRAY[0][index] = (0x80000 << 10) | 0xef;

    let index = boot_array_index(0xffff_ffff_c000_0000, 0);
    BOOT_PT_ARRAY[0][index] = (0x80000 << 10) | 0xef;
}

#[cfg(feature = "sv48")]
pub unsafe fn pre_mmu() {
    let index = boot_array_index(0x8000_0000, 0);
    BOOT_PT_ARRAY[0][index] = page_table_node(addr_of!(BOOT_PT_ARRAY[1]) as u64);
    let index = boot_array_index(0xffff_ffc0_8000_0000, 0);
    BOOT_PT_ARRAY[0][index] = page_table_node(addr_of!(BOOT_PT_ARRAY[2]) as u64);

    let index = boot_array_index(0x8000_0000, 1);
    BOOT_PT_ARRAY[1][index] = (0x80000 << 10) | 0xef;

    let index = boot_array_index(0xffff_ffc0_8000_0000, 1);
    BOOT_PT_ARRAY[2][index] = (0x80000 << 10) | 0xef;

    let index = boot_array_index(0xffff_ffff_c000_0000, 1);
    BOOT_PT_ARRAY[2][index] = (0x80000 << 10) | 0xef;
}

pub unsafe fn enable_mmu() {
    let mode = if cfg!(feature = "sv39") {
        satp::Mode::Sv39
    } else if cfg!(feature = "sv48") {
        satp::Mode::Sv48
    } else {
        panic!("Just sv39 or sv48");
    };

    let page_table_root = BOOT_PT_ARRAY[0].as_ptr() as usize;
    satp::set(mode, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

pub unsafe fn post_mmu() {
    core::arch::asm!("
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0
        add     ra, ra, t0
        ret     ",
    phys_virt_offset = const PHYS_VIRT_OFFSET,
    )
}