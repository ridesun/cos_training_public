#![no_std]
use linkme::distributed_slice;
#[distributed_slice]
pub static DRV:[fn()->Driver<'static>];
pub struct Driver<'a> {
    pub name: &'a str,
    pub compatible: &'a str,
}

impl Driver<'_> {
    pub fn info<'a>(name: &'a str, compatible: &'a str) -> Driver<'a> {
        Driver {
            name,
            compatible,
        }
    }
}
