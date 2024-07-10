pub use ircomponent::IrComponent;

pub trait IrWindow {
}

#[cfg(target_os = "windows")] mod win32;
#[cfg(target_os = "windows")] pub use win32::*;

#[cfg(target_os = "macos")] mod cocoa;
#[cfg(target_os = "macos")] pub use cocoa::*;
