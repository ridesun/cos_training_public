#![no_std]
#![no_main]

use drv_common::DRV;


#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: B0\n");
    verify();
}

fn traverse_drivers() {
    libos::println!("{}",DRV.len());
    for callentry in DRV {
        let drv=callentry();
        display_drv_info(drv.name, drv.compatible);
    }
}

fn display_initcalls_range(start: usize, end: usize) {
    libos::println!("init calls range: 0x{:X} ~ 0x{:X}\n", start, end);
}

fn display_drv_info(name: &str, compatible: &str) {
    libos::println!("Found driver '{}': compatible '{}'", name, compatible);
}

fn verify() {
    traverse_drivers();

    libos::println!("\nResult: Okay!");
}
