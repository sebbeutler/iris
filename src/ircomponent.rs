// ircomponent.rs

pub trait BeginState {
}

pub trait LaunchState {
}

pub trait IrComponent {
    fn init(&self) {
        println!("Default initializing...");
    }

    fn run(&self) {
        println!("Default running...");
    }
}
