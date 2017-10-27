use objc::runtime::{ Object };
use std;

#[allow(dead_code)]
pub fn print_nsstring(str: *mut Object) {
    use std::ffi::CStr;
    unsafe {
        let cstr: *const std::os::raw::c_char = msg_send![str, UTF8String];
        let rstr = CStr::from_ptr(cstr).to_string_lossy().into_owned();
        println!("{}", rstr);
    }
}
#[allow(dead_code)]
pub fn nsstring_decode(str: *mut Object) -> String {
    use std::ffi::CStr;
    unsafe {
        let cstr: *const std::os::raw::c_char = msg_send![str, UTF8String];
        let rstr = CStr::from_ptr(cstr).to_string_lossy().into_owned();
        rstr
    }
}