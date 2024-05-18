
pub trait IrApp {
    fn new() -> Box<Self>;

    fn run(&self);
}