pub extern crate common;
pub extern crate rmaps;

extern "C" fn init() {
    common::init_log();
    rmaps::init();
}