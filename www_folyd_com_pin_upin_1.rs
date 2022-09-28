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

    use std::pin::Pin;
    use std::marker::PhantomPinned;
    // 2nd method with PhantomPinned
    #[derive(Debug)]
    struct TestStructPinned2 {
        a: String,
        b: *const String,
        _marker: std::marker::PhantomPinned,
    }
    // first method via impl
    // #![feature(negative_impls)]
    // #[derive(Debug)]
    // struct TestStructPinned1 {
    //     a: String,
    //     b: *const String,
    // }
    // impl !Unpin for TestStructPinned1 {}

    impl TestStructPinned2 {
        fn new(text: &str) -> Self {
            TestStructPinned2 {
                a: String::from(text),
                b: std::ptr::null(),
                _marker: std::marker::PhantomPinned,
            }
        }
        fn init<'a>(self: Pin<&'a mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }
        fn a<'a>(self: Pin<&'a Self>) -> &'a str {
            &self.get_ref().a
        }
        fn b<'a>(self: Pin<&'a Self>) -> &'a String {
            unsafe { &*(self.b) }
        }
    }
    // test
    let mut tt1 = TestStructPinned2::new("test1");
    let mut t1 = unsafe { Pin::new_unchecked(&mut tt1) };
    TestStructPinned2::init(t1.as_mut());
    let mut tt2 = TestStructPinned2::new("test2");
    let mut t2 = unsafe { Pin::new_unchecked(&mut tt2) };
    TestStructPinned2::init(t2.as_mut());
    println!("a: {}, b: {}", TestStructPinned2::a(t1.as_ref()), TestStructPinned2::b(t1.as_ref()));
    // std::mem::swap(t1.get_mut(), t2.get_mut());
    println!("a: {}, b: {}", TestStructPinned2::a(t2.as_ref()), TestStructPinned2::b(t2.as_ref()));

    // test in heap
    // use std::pin::Pin;
    // use std::marker::PhantomPinned;
    struct TestStructPinned3 {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }
    impl TestStructPinned3 {
        fn new(text: &str) -> Pin<Box<Self>> {
            let t = TestStructPinned3 {
                a: String::from(text),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(t);
            let self_ptr: *const String = &boxed.as_ref().a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
            boxed
    }
    fn a<'a>(self: Pin<&'a Self>) -> &'a str { &self.get_ref().a }
    fn b<'a>(self: Pin<&'a Self>) -> &'a String { unsafe { &*(self.b) } }
    }
    // test
    dbg!("pin in heap");
    let mut t1 = TestStructPinned3::new("test1");
    let mut t2 = TestStructPinned3::new("test2");
    println!("t1 a: {}, b: {}", t1.as_ref().a(), t1.as_ref().b());
    // std::mem::swap(t1.get_mut(), t2.get_mut());
    // error[E0599]: no method named `get_mut` found for struct `Pin<Box<TestStructPinned3>>` in the current scope
    // std::mem::swap(&mut *t1, &mut *t2);
    println!("t2 a: {}, b: {}", t2.as_ref().a(), t2.as_ref().b());

/*

#[stable(feature = "futures_api", since = "1.36.0")]
 pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
 }

    // 这个async块中存在跨await的借用！
let mut fut = async {
    let to_borrow = String::from("Hello");
    let borrowed = &to_borrow;
    SomeResource::some_task().await;
    println!("{} world!", borrowed);
};

*/
/*
pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
    where T: Generator<ResumeTy, Yield = ()>,
    {
    #[rustc_diagnostic_item = "gen_future"]
    struct GenFuture<T: Generator<ResumeTy, Yield = ()>>(T);

    // We rely on the fact that async/await futures are immovable in order to create
    // self-referential borrows in the underlying generator.
    impl<T: Generator<ResumeTy, Yield = ()>> !Unpin for GenFuture<T> {}

    impl<T: Generator<ResumeTy, Yield = ()>> Future for GenFuture<T> {
        type Output = T::Return;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // SAFETY: Safe because we're !Unpin + !Drop, and this is just a field projection.
            let gen = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.0) };

            // Resume the generator, turning the `&mut Context` into a `NonNull` raw pointer. The
            // `.await` lowering will safely cast that back to a `&mut Context`.
            match gen.resume(ResumeTy(NonNull::from(cx).cast::<Context<'static>>())) {
                GeneratorState::Yielded(()) => Poll::Pending,
                GeneratorState::Complete(x) => Poll::Ready(x),
            }
        }
    }

    GenFuture(gen)
    }
 */
/*
If T: Unpin (which is the default), then Pin<'a, T> is entirely equivalent to &'a mut T. in other words: Unpin means it's OK for this type to be moved even when pinned, so Pin will have no effect on such a type.

Getting a &mut T to a pinned T requires unsafe if T: !Unpin.

Most standard library types implement Unpin. The same goes for most "normal" types you encounter in Rust. A Future generated by async/await is an exception to this rule.

You can add a !Unpin bound on a type on nightly with a feature flag, or by adding std::marker::PhantomPinned to your type on stable.

You can either pin data to the stack or to the heap.

Pinning a !Unpin object to the stack requires unsafe

Pinning a !Unpin object to the heap does not require unsafe. There is a shortcut for doing this using Box::pin.

For pinned data where T: !Unpin you have to maintain the invariant that its memory will not get invalidated or repurposed from the moment it gets pinned until when drop is called. This is an important part of the pin contract.
*/
}