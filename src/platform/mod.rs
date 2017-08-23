pub use self::platform::*;

#[cfg(target_os = "macos")]
#[path="cocoa/mod.rs"]
mod platform;
