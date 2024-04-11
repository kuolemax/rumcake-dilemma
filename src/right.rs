#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::keyboard::{build_layout, build_standard_matrix};

#[keyboard(split_peripheral(driver = "ble"))]
pub struct DilemmaRight;

use rumcake::keyboard::Keyboard;
impl Keyboard for DilemmaRight {
    // Needed for advertising data (Bluetooth GAP)
    const MANUFACTURER: &'static str = "kuolemax";
    const PRODUCT: &'static str = "Dilemma";
}

// Since this is a peripheral device, this only needs a matrix
use rumcake::keyboard::KeyboardMatrix;
impl KeyboardMatrix for DilemmaRight {
    build_standard_matrix! {
        { P0_22 P0_24 P0_29 P1_15 } // Rows
        { P0_02 P1_00 P0_11 P1_06 P1_04 } // Columns
    }

    fn remap_to_layout(row: u8, col: u8) -> (u8, u8) {
        // Since the layout is stored on the central device, we need to remap the matrix events
        // to the proper coordinates on the layout

        (row, 5 + col)
    }
}

// Bluetooth configuration
use rumcake::hw::mcu::BluetoothDevice;
impl BluetoothDevice for DilemmaRight {
    const BLUETOOTH_ADDRESS: [u8; 6] = [0xA8, 0xB2, 0x4F, 0x6E, 0x0D, 0x7C];
}

// Split keyboard setup
impl NRFBLEPeripheralDriverSettings for DilemmaRight {
    const CENTRAL_ADDRESS: [u8; 6] = [0x5C, 0x3A, 0x7D, 0x9F, 0x2E, 0x1B];
}
