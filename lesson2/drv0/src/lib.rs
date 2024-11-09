#![no_std]

use drv_common::{Driver,DRV};
use linkme::distributed_slice;

#[distributed_slice(DRV)]
fn drv0_init_fn() -> Driver<'static> {
    Driver::info("rtc", "google,goldfish-rtc")
}