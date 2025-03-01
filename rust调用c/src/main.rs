extern crate libc;

unsafe extern "C" {
    fn double_input(input: libc::c_int) -> libc::c_int;
    fn third_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 4;
    let output = unsafe { double_input(input) };
    let output2: i32 = unsafe { third_input(input) };
    println!("{} * 3 = {}", input, output2);
    println!("{} * 2 = {}", input, output);
}
