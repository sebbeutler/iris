#![cfg(target_os = "macos")]

use crate::IrWindow;

pub struct IrWindowCocoa {
}

impl IrWindowCocoa {
}

impl IrWindow for IrWindowCocoa {
    fn init_window(&self) {
    }
}
