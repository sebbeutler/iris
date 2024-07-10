
pub trait Initialize {
    fn init(&self);
}

pub trait Runnable {
    fn run(&self);
}

pub trait IrComponent : Runnable + Initialize {
    fn init(&self) {
        println!("Default initializing...");
    }

    fn run(&self) {
        println!("Default running...");
    }
}

impl<T: IrComponent> Runnable for T {
    fn run(&self) {
        IrComponent::run(self);
    }
}

impl<T: IrComponent> Initialize for T {
    fn init(&self) {
        IrComponent::init(self);
    }
}
