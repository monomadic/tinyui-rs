pub use self::platform::*;

#[cfg(target_os = "macos")]
#[path="cocoa/mod.rs"]
mod platform;

#[cfg(target_os = "windows")]
panic!("library not supported on windows");
