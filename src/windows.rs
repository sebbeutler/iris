#![cfg(windows)]
use irwindow;

pub fn launch() {
    let hwnd = irwindow::create_main_window("iris", "iris")
        .expect("Iris window creation failed!");
    irwindow::run(hwnd);
}
