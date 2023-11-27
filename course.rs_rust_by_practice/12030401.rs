fn foo() -> i32 { 0 }

fn main() {
    let ptr = foo as *const ();
    let func = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(ptr)
    };

    assert_eq!(func(), 0);
}
