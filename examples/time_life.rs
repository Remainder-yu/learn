use std::{slice::from_raw_parts,str::from_utf8_unchecked};

fn get_memory_location() -> (usize, usize) {
    let string = "hello world!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer,length)
}

fn get_str_at_locaton(pointer:usize, length: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked((from_raw_parts(pointer as *const u8, length)))
    }
}

fn main() {
    let (pointer, length) = get_memory_location();
    let message = get_str_at_locaton(pointer, length);
    println!("the {} bytes at 0x{:X} stored: {}",length,pointer,message);
}


#[derive(Debug)]
struct T {
    member: i32,
}

fn test<'a>(arg: &'a T) -> &'a i32{
    &arg.member
}

fn test_main(){
    let t = T { member: 0 };
    let x = test(&t);
    println!("{:?}",x);
    println!("{:?}",t);
}