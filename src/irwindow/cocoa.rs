// cocoa.rs

use crate::irwindow::IrWindow;
use crate::IrComponent;

pub struct IrWindowCocoa {
}

impl IrWindowCocoa {
}

impl IrWindow for IrWindowCocoa {

}

impl IrComponent for IrWindowCocoa {
    fn init(&self) {
        println!("## Initializing Cocoa Window 🪟")
    }
}
