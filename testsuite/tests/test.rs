#![no_std]
#![no_main]

use cortex_m_rt::entry;
use {{crate_name}} as _; // memory layout + panic handler

// See https://crates.io/crates/defmt-test/0.1.0 for more documentation
#[defmt_test::tests]
mod tests {
    #[test]
    fn assert_true() {
        assert!(true)
    }

    #[test]
    fn assert_false() {
        assert!(false, "TODO: write actual tests")
    }
}
