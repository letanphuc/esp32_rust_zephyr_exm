// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

#[cfg(feature = "executor-thread")]
use embassy_executor::Executor;

#[cfg(feature = "executor-zephyr")]
use zephyr::embassy::Executor;

use embassy_executor::Spawner;
use static_cell::StaticCell;
use zephyr::kconfig::CONFIG_BOARD;

static EXECUTOR_MAIN: StaticCell<Executor> = StaticCell::new();

use log::warn;
use zephyr::raw::ZR_GPIO_OUTPUT_ACTIVE;
use zephyr::time::{sleep, Duration};

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    log::info!("Starting Embassy executor on {}", CONFIG_BOARD);

    let executor = EXECUTOR_MAIN.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(main(spawner)).unwrap();
    })
}

async fn sleep_ms(ms: u64) {
    sleep(Duration::millis_at_least(ms));
}

#[embassy_executor::task]
async fn main(_spawner: Spawner) {
    warn!("Inside of blinky");

    let mut led0 = zephyr::devicetree::aliases::led0::get_instance().unwrap();

    if !led0.is_ready() {
        warn!("LED is not ready");
        loop {}
    }

    led0.configure(ZR_GPIO_OUTPUT_ACTIVE);

    loop {
        log::info!("Toggling LED");
        led0.toggle_pin();

        sleep_ms(100).await;
    }
}
