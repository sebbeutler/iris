use irwindow::*;

fn main() {
    let hwnd = create_main_window("my_window", "Example window creation")
        .expect("Window creation failed!");
    run(hwnd);
}