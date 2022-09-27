#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
// demo for std::mem::swap to swap two String objects
    let mut x = String::from("xxx");
    let mut y = String::from("yyy");

    std::mem::swap(&mut x, &mut y);

    assert_eq!("yyy", &x);
    assert_eq!("xxx", &y);

// Self-Referential Structs
// demo to construct a structure that the memeber link another String's reference
// it will dose not work
struct Test<'a> {
    a: String,
    b: &'a String,
}
    let a = String::from("hello");
    // let _test = Test {a, b: &a };
/*
error[E0382]: borrow of moved value: `a`
 --> www_folyd_com_pin_upin_1.rs:8:29
  |
7 |     let a = String::from("hello");
  |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
8 |     let _test = Test {a, b: &a };
  |                       -     ^^ value borrowed here after move
  |                       |
  |                       value moved here
*/

// demo if intrinsic type implement 'Copy' trait, like integer type u* or i*?
// it works because int32 implement 'Copy' trait
#[derive(Debug)]
struct TestIntType<'a> {
    a: i32,
    b: &'a i32,
}
    let a: i32 = 0;
    let _test_int32 = TestIntType {a, b: &a };
    println!("Integer32 implement 'Copy' trait, so it works:\n\t{:?}", _test_int32);

// workaround to solve problem 'Self-Referential Struct'
#[derive(Debug)]
struct TestSelfReferentialStruct {
    a: String,
    b: *const String, // raw ptr to String
}
impl TestSelfReferentialStruct {
    fn new(text: &str) -> Self {
        TestSelfReferentialStruct {
            a: String::from(text),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        // let self_ref: *const String = &self.a;
        // self.b = self_ref;
        self.b = &self.a as *const String;
    }

    fn a(&self) -> &str { &self.a }
    fn b(&self) -> &String { unsafe { &*(self.b)}}
}

    let mut test_srs1 = TestSelfReferentialStruct::new("test1");
    test_srs1.init();
    let mut test_srs2 = TestSelfReferentialStruct::new("test2");
    test_srs2.init();

    println!("before std::mem::swap()");
    println!("test_srs1: String value a: \t{:?}, value a's pointer address b: \t{:?}", test_srs1.a, test_srs1.b);
    println!("value a's content: \t{:p}, is ptr to heap", &test_srs1.a);
    println!("ptr &a's address: \t{:p}", &&test_srs1.a);
    println!("value b's content: \t{:?}, is ptr to a's mem", test_srs1.b);
    println!("ptr b's address: \t{:p}", &test_srs1.b);

    println!("test_srs2: String value a: \t{:?}, value a's pointer address b: \t{:?}", test_srs2.a, test_srs2.b);
    println!("value a's content: \t{:p}, is ptr to heap", &test_srs2.a);
    println!("ptr &a's address: \t{:p}", &&test_srs2.a);
    println!("value b's content: \t{:?}, is ptr to a's mem", test_srs2.b);
    println!("ptr b's address: \t{:p}", &test_srs2.b);

    // swap(), move
    std::mem::swap(&mut test_srs1, &mut test_srs2);
    println!("after std::mem::swap()");
    println!("test_srs1: String value a: \t{:?}, value a's pointer address b: \t{:?}", test_srs1.a, test_srs1.b);
    println!("value a's content: \t{:p}, is ptr to heap", &test_srs1.a);
    println!("ptr &a's address: \t{:p}", &&test_srs1.a);
    println!("value b's content: \t{:?}, is ptr to a's mem", test_srs1.b);
    println!("ptr b's address: \t{:p}", &test_srs1.b);

    println!("test_srs2: String value a: \t{:?}, value a's pointer address b: \t{:?}", test_srs2.a, test_srs2.b);
    println!("value a's content: \t{:p}, is ptr to heap", &test_srs2.a);
    println!("ptr &a's address: \t{:p}", &&test_srs2.a);
    println!("value b's content: \t{:?}, is ptr to a's mem", test_srs2.b);
    println!("ptr b's address: \t{:p}", &test_srs2.b);
    println!("");
    println!("test_srs1: String value a: {:10}, b(ptr): {:p}, *b: {}", test_srs1.a(), test_srs1.b(), test_srs1.b());
    println!("test_srs2: String value a: {:10}, b(ptr): {:p}, *b: {}", test_srs2.a(), test_srs2.b(), test_srs2.b());
    // set test1's a value to new, check the address of a
    // test_srs1.a = "I've totally changed now!".to_string();
    test_srs1.a = "changed!".to_string();
    println!("after set test1's a");
    println!("test_srs1: String value a: {:10}, b(ptr): {:p}, *b: {}", test_srs1.a(), test_srs1.b(), test_srs1.b());
    println!("test_srs2: String value a: {:10}, b(ptr): {:p}, *b: {}", test_srs2.a(), test_srs2.b(), test_srs2.b());
    // println!({}, )



}