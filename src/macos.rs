#![cfg(target_os = "macos")]

use crate::irbase::IrApp;
use irwindow::IrWindowCocoa;

pub struct IrAppMacOS;
impl IrApp for IrAppMacOS {
    fn new() -> Box<Self> {
        let app = IrAppMacOS {};
        let window = IrWindowCocoa {};
        Box::new(app)
    }

    fn run(&self) {

    }
}