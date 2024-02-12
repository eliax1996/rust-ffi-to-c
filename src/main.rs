extern crate core;
use core::ffi::c_char;
use core::ffi::c_void;

extern "C" {
    fn ops(a: *const c_char) -> c_void;
}

struct Obj([u8; 8]);

fn sub_fn() {

    println!(
        "Obj occupies {} bytes on the stack.",
        std::mem::size_of::<Obj>()
    );

    let s = Obj([1,2,3,4,5,6,7,8]);
    println!("s pointer: {:p}", &s);
    unsafe {
       let r = &s as *const _ as *const c_char;
       ops(r);
    };
    std::mem::drop(s);
    let r = Obj([11,12,13,14,15,16,17,18]);
    println!("r pointer: {:p}", &r);

    std::mem::drop(r);
}

fn main()
{
  sub_fn();
  sub_fn();
  sub_fn();
  sub_fn();
  sub_fn();
  sub_fn();
}