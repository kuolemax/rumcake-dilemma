#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::keyboard::remap_matrix;
use rumcake::keyboard::{build_layout, build_standard_matrix};

#[keyboard(usb, bluetooth, split_central(driver = "ble"))]
pub struct DilemmaLeft;

// Basic keyboard configuration
use rumcake::keyboard::Keyboard;
impl Keyboard for DilemmaLeft {
    const MANUFACTURER: &'static str = "kuolemax";
    const PRODUCT: &'static str = "Dilemma";
}

// Layout configuration
use rumcake::bluetooth::BluetoothCommand::*;
use rumcake::keyberon::action::{Action::Custom, Action::*};
use rumcake::keyboard::{KeyboardLayout, Keycode::*};
impl KeyboardLayout for DilemmaLeft {
    build_layout! {
        {
            [ Q      W  E    R  T   Y  U  I      O  P     ]
            [ A      S  D    F  G   H  J  K      L  ;     ]
            [ Z      X  C    V  B   N  M  ,      .  /     ]
            [ LShift No LAlt No No  No No BSpace No Space ]
        }
    }
}

// Matrix configuration
use rumcake::keyboard::KeyboardMatrix;
impl KeyboardMatrix for DilemmaLeft {
    build_standard_matrix! {
        { P0_22 P0_24 P0_29 P1_15 } // Rows
        { P1_04 P1_06 P0_11 P1_00 P0_02 } // Columns
    }
}

// Bluetooth configuration
use rumcake::hw::mcu::BluetoothDevice;
impl BluetoothDevice for DilemmaLeft {
    const BLUETOOTH_ADDRESS: [u8; 6] = [0x5C, 0x3A, 0x7D, 0x9F, 0x2E, 0x1B];
}

use rumcake::bluetooth::BluetoothKeyboard;
impl BluetoothKeyboard for DilemmaLeft {
    const BLE_VID: u16 = 0xACF0;
    const BLE_PID: u16 = 0xDB01;
}

// Split keyboard setup
impl NRFBLECentralDriverSettings for DilemmaLeft {
    const PERIPHERAL_ADDRESSES: &'static [[u8; 6]] = &[[0xA8, 0xB2, 0x4F, 0x6E, 0x0D, 0x7C]];
}

// USB configuration
use rumcake::usb::USBKeyboard;
impl USBKeyboard for DilemmaLeft {
    const USB_VID: u16 = 0xACF0;
    const USB_PID: u16 = 0xDD01;
}
