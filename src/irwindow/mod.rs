// irwindow.rs (mod)

pub use crate::IrComponent;

// TODO: Maybe use a irwindow wrapper for platform specific windows.
pub trait IrWindow : IrComponent {
}

#[cfg(target_os = "windows")] mod win32;
#[cfg(target_os = "windows")] pub use win32::*;

#[cfg(target_os = "macos")] mod cocoa;
#[cfg(target_os = "macos")] pub use cocoa::*;

pub fn new() -> Box<dyn IrWindow> {
    #[cfg(target_os = "macos")]
    return Box::new(IrWindowCocoa {});

    #[cfg(target_os = "windows")]
    Box::new(IrWindowWin32 {})
}
