use irwindow::IrWindowWin32;

pub struct IrAppWindows;
pub impl IrApp for IrAppWindows {
    fn new() -> Box<Self> {
        let app = IrAppMacOS {};
        let window = IrWindowCocoa {};
        Box::new(app)
    }

    fn run(&self) {

    }
}
