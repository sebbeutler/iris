#![cfg(windows)]

pub fn launch() {
    let hwnd = create_main_window("iris", "iris")
        .expect("Iris window creation failed!");
    run(hwnd);
}
