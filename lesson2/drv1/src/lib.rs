#![no_std]

use drv_common::{Driver,DRV};
use linkme::distributed_slice;

#[distributed_slice(DRV)]
fn drv1_init_fn() -> Driver<'static> {
    Driver::info("uart", "ns16550a")
}