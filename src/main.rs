use std::io::Error;

mod irbase;
use irbase::IrApp;

#[cfg(target_os = "windows")] mod windows;
#[cfg(target_os = "macos")] mod macos;

fn main() -> Result<(), Error>{
    
    #[cfg(target_os = "windows")]
    let app = macos::IrAppMacOS::new();
    #[cfg(target_os = "macos")]
    let app = macos::IrAppMacOS::new();

    app.run();

    Ok(())
}
