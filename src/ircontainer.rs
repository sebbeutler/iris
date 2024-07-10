use crate::IrComponent;

pub struct IrContainer;

impl IrComponent for IrContainer {
    fn init(&self) {
        println!("Container initializing...");
    }

    fn run(&self) {
        println!("Container running...");
    }
}