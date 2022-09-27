// [查看变量在内存中的存储结构 - Rust - 知乎](https://zhuanlan.zhihu.com/p/102591451)
// [Using unsafe tricks to examine Rust data structure layout](https://pramode.in/2016/09/13/using-unsafe-tricks-in-rust/)

fn main() {
    // int
    let a = 7;
    let b = &a;
    println!("{:p}", &a);
    println!("{:p}", &&a);
    println!("{:p}", &&&a);
    println!("{:?}", b as *const i32);

    // check mem::size_of
    assert_eq!(4, std::mem::size_of::<i32>());
    assert_eq!(8, std::mem::size_of::<i64>());
    assert_eq!(0, std::mem::size_of::<()>());

    // auto dereference
    let ori = 7;
    let borrowed = &ori;
    let boxed = Box::new(ori);
    let rced = std::rc::Rc::new(ori);
    println!("ptr: {}, {}, {}", borrowed, boxed, rced);
    println!("dereference: {}, {}, {}", *borrowed, *boxed, *rced);

    // raw ptr
    let a = 7;
    let b = &a as *const i32; // convert to raw ptr is free, but use raw ptr need unsafe block
    println!("{:p}", &a);
    println!("{:?}", b);
    // println!("{:?}", *b);
    unsafe { println!("{:?}", *b); }
/*
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
  --> www_rust_check_data_layout_ptr.rs:29:22
   |
29 |     println!("{:?}", *b);
   |                      ^^ dereference of raw pointer
   |
   = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
*/

    // check bits
    // let input = 2049;
    //let input: u32 = 0x10111213;
    let input: u32 = 0x04030201;
    print_bytes(&input);

    fn print_bytes(input: &u32) {
        let ptr_u32: *const u32 = input; // convert &u32 to u32 raw ptr
        let ptr_u8 = ptr_u32 as *const u8; // convert u32 raw ptr to u8 raw ptr
        unsafe {
            println!("{:02x}-{:02x}-{:02x}-{:02x}",
                     *ptr_u8,
                     *ptr_u8.offset(1),
                     *ptr_u8.offset(2),
                     *ptr_u8.offset(3));
        }
    }

    //[pointer - Rust](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset)
    let s: &str = "123";
    let ptr: *const u8 = s.as_ptr();
    // let ptr = s as *const u8; // error[E0606]: casting `&str` as `*const u8` is invalid
    unsafe {
        println!("{}", *ptr as char);
        println!("{}", *ptr.offset(1) as char);
        println!("{}", *ptr.offset(2) as char);
    }

    // transmute
    let a: u32 = 0x04030201;
    // let b: [u8; 4];
    // b = unsafe { std::mem::transmute(a) };
    let b = unsafe { std::mem::transmute::<u32, [u8;4]>(a) };
    println!("{:02x}-{:02x}-{:02x}-{:02x}", b[0], b[1], b[2], b[3]);

    // check mem layout
    // Structure
    struct Unit {
        _id: u8,
        _num: u8,
    }
    println!("Size of 2 bytes Unit: {}", std::mem::size_of::<Unit>());
    let unit = Unit { _id: 0, _num: 1 };
    let mem = unsafe { std::mem::transmute::<Unit, [u8; 2]>(unit) };
    println!("[u8; 2]: {:}, {:}", mem[0], mem[1]);

    // Rc<T>
    let ori: usize = 7;
    let rc = std::rc::Rc::new(ori);
    println!("size of Rc<u8>: {}", std::mem::size_of::<std::rc::Rc<usize>>());
    println!("ori address:\t{:p}", rc); // value ori -> addr in heap
    let mem: usize;
    mem = unsafe { std::mem::transmute::<std::rc::Rc<usize>, usize>(rc)};
    println!("value in stack:\t0x{:x}", mem);

    // check Rc layout
    println!("\nbefore clone:");
    let ori: usize = 0;
    let rc = std::rc::Rc::new(ori);
    println!("size of Rc<u8>: {}", std::mem::size_of::<std::rc::Rc<usize>>());
    println!("&rc:\t\t{:p}", &rc); // value ori in heap addr
    println!("ori address:\t{:p}", rc); // value ori in heap addr
    let mut mem: usize;
    mem = unsafe { std::mem::transmute_copy(&rc) };
    println!("value in stack:\t0x{:x}", mem);
    //check the content in heap
    let rc_ptr: *const usize;
    rc_ptr = mem as *const usize;
    unsafe {
        println!("strong: {}, weak: {}, ori: {}", 
                 *rc_ptr,               // strong 8B
                 *rc_ptr.offset(1),     // weak 8B
                 *rc_ptr.offset(2),     // usiz 8B
                );
    }
    let clone = std::rc::Rc::clone(&rc);
    println!("\nafter clone:");
    println!("ori addr:\t{:p}", clone);
    mem = unsafe { std::mem::transmute_copy(&rc) };
    println!("Value in stack\t0x{:x}", mem);
    unsafe {
        println!("strong: {}, weak: {}, ori: {}", 
                 *rc_ptr,
                 *rc_ptr.offset(1),
                 *rc_ptr.offset(2),
                );
    }
/*
+----------------|
|                |
+----------------+
| 0x5579a8174b50 +----v
+----------------+    |
|                |    |
|                |    |
+----------------+    |
                      v
                  +---------------+---------+-----------+---------+
                  |   | strong: 2 | weak: 1 | ori: 1234 |         |
                  +---+-----------+---------+-----------+---------+
 */

    // HashMap
    // check size
    use std::collections::HashMap;
    use std::mem;
    dbg!("HashMap");
    println!("{}", std::mem::size_of::<HashMap<usize, String>>());

    let map: HashMap<usize, String> = HashMap::new();
    let mem: [usize; 6];
    mem = unsafe { std::mem::transmute_copy(&map) };
    println!("{:x}-{:x}-{:x}-{:x}-{:x}-{:x}", mem[0], mem[1], mem[2], mem[3], mem[4], mem[5]);


        // as_ptr()
        let s = String::from("012345");

        println!("{:p} on stack", &s);

        let (ptr_stack, cap, len)= unsafe { mem::transmute_copy::<String, (usize, usize, usize)>(&s) };
        println!("String layout:\nheap addr: 0x{:x} cap: {} len: {}", ptr_stack, cap, len);

        // let s = String::from("012345");
        let ptr_heap = s.as_ptr();
        //println!("{:p} on heap", s.as_ptr());
        println!("{:p} on heap", ptr_heap);
        // let ptr_raw = ptr_heap as *const u8;
        let ptr_raw = ptr_heap;
        unsafe {
                println!("heap\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}",
                            ptr_raw.offset(0), *ptr_raw.offset(0),
                            ptr_raw.offset(1), *ptr_raw.offset(1),
                            ptr_raw.offset(2), *ptr_raw.offset(2),
                            ptr_raw.offset(3), *ptr_raw.offset(3),
                            ptr_raw.offset(4), *ptr_raw.offset(4),
                            ptr_raw.offset(5), *ptr_raw.offset(5),
                            ptr_raw.offset(6), *ptr_raw.offset(6),
                    );
        }

        // try to get String's stack start ptr
        /*
        */
        dbg!("try to get String's stack start ptr");
        let s = String::from("0123456789");
        println!("{:p} on stack", &s);
        //#[feature(strict_provenance)]
        //let ptr_usize = std::ptr::addr(&s);
        // let ptr_usize = core::pointer::addr(&s);
        // let ptr_raw = &s as *const _ as *const usize;
        let ptr_raw = std::ptr::addr_of!(s) as *const usize;
        unsafe {
            println!("String start addr in stack: {:?}\n\
                            addr: {:?} val: 0x{:x}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}",
                    ptr_raw,
                    ptr_raw.offset(0), *ptr_raw.offset(0),
                    ptr_raw.offset(1), *ptr_raw.offset(1),
                    ptr_raw.offset(2), *ptr_raw.offset(2),

            );
        }
        let (ptr_stack, cap, len)= unsafe { mem::transmute_copy::<String, (usize, usize, usize)>(&s) };
        println!("String layout:\nheap addr: 0x{:x} cap: {} len: {}", ptr_stack, cap, len);
        let ptr_heap = s.as_ptr();
        //println!("{:p} on heap", s.as_ptr());
        println!("{:p} on heap", ptr_heap);
        // let ptr_raw = ptr_heap as *const u8;
        let ptr_raw = ptr_heap;
        unsafe {
                println!("heap\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}\n\
                            addr: {:?} val: {:?}",
                            ptr_raw.offset(0), std::str::from_utf8(&[*ptr_raw.offset(0)]).unwrap(),
                            ptr_raw.offset(1), *ptr_raw.offset(1) as char,
                            ptr_raw.offset(2), *ptr_raw.offset(2) as char,
                            ptr_raw.offset(3), *ptr_raw.offset(3) as char,
                            ptr_raw.offset(4), *ptr_raw.offset(4) as char,
                            ptr_raw.offset(5), *ptr_raw.offset(5) as char,
                            ptr_raw.offset(6), *ptr_raw.offset(6) as char,
                            ptr_raw.offset(7), *ptr_raw.offset(7) as char,
                            ptr_raw.offset(8), *ptr_raw.offset(8) as char,
                            ptr_raw.offset(9), *ptr_raw.offset(9) as char,
                    );
        }

// [Using unsafe tricks to examine Rust data structure layout](https://pramode.in/2016/09/13/using-unsafe-tricks-in-rust/)
    dbg!("Using unsafe tricks to examine Rust data structure layout");
    // Get the addr of a variable
    dbg!("auto deref and ref, raw ptr");
    // auto dereference
    let a = 10;
    let b = &a;
    // let c: *const i32 = &a as *const i32;
    // let c = &a as *const i32;
    let c = &a as *const _;
    println!("b: {:?}, *b: {:?}, b+1: {:?}, *b+1: {:?}", b, *b, b+1, *b+1);
    println!("b, ref ptr via {{b:p}}: {:p}", b);
    println!("b, ref ptr via {{b:?}}: {:?}, `as *const _` cast to raw ptr", b as *const _); // as *const i32
    println!("c, ref ptr via {{c:?}}: {:?}", c);

    dbg!("deref raw ptr");
    let p = 0 as *const i32; // p is raw ptr and point to 0x0
    println!("{:?}", p);
    // let k = *p;
    unsafe {
        let k = *p;
        // println!("{:?}", k); // Segmentation fault: 11
    }

    dbg!("mem layout int32");
    fn print_bytes1(p: &u32) {
        let q = p as *const u32;
        let r = q as u64;
        for i in 0..4 {
            unsafe { print!("{:x}, ", *((r + i) as *const u8)); }
        }
        println!();
    }
    fn print_bytes2(p: &u32) {
        let rptr = std::ptr::addr_of!(*p) as *const u8;
        // let rptr = p as *const _ as *const u8;
        for i in 0..=3 {
            unsafe { print!("{:x}, ", *rptr.offset(i)); }
            // unsafe { print!("{:x}, ", *rptr); }
        }
        println!();
    }
    fn print_bytes3(p: &u32) {
        // let rptr = std::ptr::addr_of!(*p) as *const u8;
        let rptr = p as *const _ as *const u8;
        let size = std::mem::size_of_val(p);
        for i in 0..size {
            unsafe { print!("{:x}, ", *rptr.offset(i as isize)); }
            // unsafe { print!("{:x}, ", *rptr); }
        }
        println!();
    }
    let a: u32 = 0x12345678;
    print_bytes1(&a);
    print_bytes2(&a);
    print_bytes3(&a);

    dbg!("transmute");
    let a = 0x12345678_u32;
    // let b = unsafe { std::mem::transmute::<u32, [u8; 4]>(a) };
    // let b: [u8; 4] = unsafe { std::mem::transmute(a) };
    let b: [u8; 4] = unsafe { std::mem::transmute_copy(&a) };
    println!("{:x}, {:x}, {:x}, {:x}", b[0], b[1], b[2], b[3]);

    dbg!("imute and mut");
    let          x: i32 = 0;
    let mut      y: i32 = 0;

    let        xx1: &i32 = &0;
    println!("let xx1: &i32 = {}", *xx1);
    let ref     xx: i32 = 0;
    println!("let ref xx: i32 = {}", *xx);

    let ref mut yy0: i32 = 0;
    let ref mut yy = 0;
    *yy = 1;
    // yy = yy0;

    let    mut yy1: &mut i32 = &mut 0;
    println!("{:?}", yy1); // 0
    *yy1 = 2;
    println!("{:?}", yy1); // 2
    yy1 = yy;
    println!("{:?}", yy1); // 1

    dbg!("raw ptr");
    let zero = 0i32;
    let one = 1i32;
    let two = 2i32;
    let three = 3i32;
    let         z1: *const i32 = std::ptr::addr_of!(zero);
    let     mut z2: *const i32 = std::ptr::addr_of!(one);
    unsafe { println!("addr: {:?}, val: {:?}", z1, *z1); }
    unsafe { println!("addr: {:?}, val: {:?}", z2, *z2); }
    z2 = z1;
    unsafe { println!("addr: {:?}, val: {:?}", z2, *z2); }
    let         z3: *mut i32 = std::ptr::addr_of!(two) as *mut i32;
    let     mut z4: *mut i32 = std::ptr::addr_of!(three) as *mut i32;
    unsafe { println!("addr: {:?}, val: {:?}", z3, *z3); }
    unsafe { *z3 = 13; println!("addr: {:?}, val: {:?}", z3, *z3); }
    unsafe { println!("addr: {:?}, val: {:?}", z4, *z4); }
    unsafe { z4 = z3; println!("addr: {:?}, val: {:?}", z4, *z4); }
    dbg!("size_of_val");
    // println!("{:?}", std::mem::size_of_val(yy));
    println!("{:?}", std::mem::size_of::<&i32>());
    println!("{:?}", std::mem::size_of::<&mut i32>());
    println!("{:?}", std::mem::size_of::<*const i32>());
    println!("{:?}", std::mem::size_of::<*mut   i32>());

    dbg!("vec layout");
    let p: [u64; 3];
    let mut v: Vec<i32> = vec![];
    v.push(10);
    println!("{:?}", &v[0] as *const i32);
    p = unsafe { std::mem::transmute(v) };
    println!("0x{:x}, {:x}, {:x}", p[0], p[1], p[2]);

    let q:[u64; 3] = [0, 1, 1];
    let mut w: Vec<i32> = vec![];
    w = unsafe { std::mem::transmute(q) };
    // println!("{:?}", w[0]); // Segmentation fault: 11
    println!("mem::size_of::<Vec<i32>>() {:?}", mem::size_of::<Vec<i32>>());
    println!("mem::size_of_val(w: &Vec) {:?}", mem::size_of_val(&w));

    dbg!("slice layout");
    let v: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let t = &v[3..7];
    let r: [u64; 2];
    println!("{:?}", &v[3] as *const i32);
    r = unsafe { std::mem::transmute(t) };
    println!("0x{:x}, {:x}", r[0], r[1]);

    println!("let t = &v[3..7]; t: {:p}", t);
    println!("slice ref start: {:?}\n\
              slice ref len: {:?}\n\
              ref 1st u64: \t{:?}\n\
              ref 1st u64 val:0x{:x}\n\
              ref 2nd u64: \t{:?}\n\
              ref 2nd u64 val:{:?}",
            std::ptr::addr_of!(t),
            std::mem::size_of_val(t),
            unsafe { (std::ptr::addr_of!(t) as *const usize) },
            unsafe { *(std::ptr::addr_of!(t) as *const usize) },
            unsafe { (std::ptr::addr_of!(t) as *const usize).offset(1) },
            unsafe { *(std::ptr::addr_of!(t) as *const usize).offset(1) },
        );

    fn print_bytes4(p: &[i32]) {
        // let rptr = std::ptr::addr_of!(*p) as *const u8;
        let rptr = p as *const _ as *const i32;
        let size = std::mem::size_of_val(p) / std::mem::size_of::<i32>();
        println!("show mem layout:");
        for i in 0..size {
            unsafe { print!("{:?}: {:x}\n", rptr.offset(i as isize), *rptr.offset(i as isize)); }
            // unsafe { print!("{:x}, ", *rptr); }
        }
        println!();
    }
    print_bytes4(&v[..]);

    dbg!("sum type");
    enum Color {
        Red,
        Green,
        Blue,
    }
    println!("{}", std::mem::size_of::<Color>());
    let color_red = Color::Red;
    let color_green = Color::Green;
    let color_blue = Color::Blue;
    // println!("{:?}", color_red);
    let digit_red: u8 = unsafe { std::mem::transmute(color_red) };
    let digit_green: u8 = unsafe { std::mem::transmute(color_green) };
    let digit_blue: u8 = unsafe { std::mem::transmute(color_blue) };
    println!("{:?} {:?} {:?}", digit_red, digit_green, digit_blue);

    enum Color2 {
        Red(u32),
        Green(u32),
        Blue(u32),
    }
    println!("enum Color2::Red(u32) size: {}", std::mem::size_of::<Color2>());
    let cred = Color2::Red(0x12345678);
    let cgrn = Color2::Green(0x0);
    let cblu = Color2::Blue(0x999999);
    let dred: [u32; 2] = unsafe { std::mem::transmute(cred) };
    let dgrn: [u32; 2] = unsafe { std::mem::transmute(cgrn) };
    let dblu: [u32; 2] = unsafe { std::mem::transmute(cblu) };
    println!("red: {:x} {:x}\ngrn: {:x} {:x}\nblu: {:x} {:x}", dred[0], dred[1], dgrn[0], dgrn[1], dblu[0], dblu[1]);

}
