#![no_std]

#![feature(alloc)]

extern crate uefi;
extern crate uefi_services;

#[macro_use]
extern crate log;
#[macro_use]
extern crate alloc;

mod boot;

use uefi::{Handle, Status};
use uefi::table;

#[no_mangle]
pub extern "C" fn uefi_start(handle: Handle, st: &'static table::SystemTable) -> Status {
    uefi_services::init(st);

    let stdout = st.stdout();
    stdout.reset(false).unwrap();

    // Switch to the maximum supported graphics mode.
    let best_mode = stdout.modes().last().unwrap();
    stdout.set_mode(best_mode).unwrap();

    info!("# uefi-rs test runner");
    info!("Image handle: {:?}", handle);

    // Test the memory allocator.
    {
        let mut values = vec![-5, 16, 23, 4, 0];

        values.sort();

        info!("Sorted vector: {:?}", values);
    }

    {
        let revision = st.uefi_revision();
        let (major, minor) = (revision.major(), revision.minor());

        info!("UEFI {}.{}.{}", major, minor / 10, minor % 10);
    }

    let bt = st.boot;

    match boot::boot_services_test(bt) {
        Ok(_) => info!("Boot services test passed."),
        Err(status) => error!("Boot services test failed with status {:?}", status),
    }

    bt.stall(4_000_000);

    let rt = st.runtime;
    rt.reset(table::runtime::ResetType::Shutdown, Status::Success, None);
}
