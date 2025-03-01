// extern crate libc;
// use std::ffi::{CStr, CString};
// #[no_mangle]
// pub extern "C" fn rustdemo(name: *const libc::c_char) -> *const libc::c_char {
//     let cstr_name = unsafe { CStr::from_ptr(name) };
//     let mut str_name = cstr_name.to_str().unwrap().to_string();
//     println!("Rust get Input:  \"{}\"", str_name);
//     let r_string: &str = " Rust say: Hello Go ";
//     str_name.push_str(r_string);
//     CString::new(str_name).unwrap().into_raw()
// }

// #![crate_type = "staticlib"] // 指定rustc编译成什么库类型，这里指定为静态库类型。
// use std::ffi::{CStr, CString};
// #[no_mangle]
// pub extern "C" fn rust_say_hello(name: *const libc::c_char) -> *const libc::c_char {
//     let cstr_name = unsafe { CStr::from_ptr(name) };
//     let mut str_name = cstr_name.to_str().unwrap().to_string();
//     println!("Rust get Input:  \"{}\"", str_name);
//     let r_string: &str = " Rust say: Hello Go ";
//     str_name.push_str(r_string);
//     CString::new(str_name).unwrap().into_raw()
// }
extern crate libc;
use std::ffi::{CStr, CString};
#[unsafe(no_mangle)]
pub extern "C" fn rustdemo(name: *const libc::c_char) -> *const libc::c_char {
    let cstr_name = unsafe { CStr::from_ptr(name) };
    let mut str_name = cstr_name.to_str().unwrap().to_string();
    println!("Rust get Input:  \"{}\"", str_name);
    let r_string: &str = " Rust say: Hello Go ";
    str_name.push_str(r_string);
    CString::new(str_name).unwrap().into_raw()
}
