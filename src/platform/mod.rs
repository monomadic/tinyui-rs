#[cfg(target_os = "macos")]
#[path="cocoa/mod.rs"]
pub mod platform;

#[cfg(target_os = "windows")]
panic!("library not supported on windows");

pub use self::platform::*;
