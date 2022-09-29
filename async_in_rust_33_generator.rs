/*
A generator is a "resumable function"
primary feature of a generator is that
it can be suspended during execution to be resumed at a later date.
Generators use the yield keyword to "return", and then
the caller can resume a generator to resume execution just after the yield keyword.
模型：<resume-work-[input->yield(end)]->output>-<>-...<end>
1. 生成器，目标是，在函数整个过程中，生成断点，断点有暂停、恢复两个方法，暂停可产出数据，终止有条件和状态
2. 生成器，恢复无入有出，暂停有入无出
3. 断点产出的值的类型在第一个断点时确定，产出的值，由间隔的过程决定
4. 结束点的类型和值，由函数最后的表达式返回值类型和值，或者ret决定
5. 如果生成器特殊内化，去掉头尾，间隔过程一致，拥有初始化的自身数据，那么产出可由内部控制，结束条件也由内部控制，转化为一个内部逻辑决定生成值的生成器
6. 内化的生成器，也是一个自我迭代器，初始化数据+终止条件决定数据产生的值
7. 生成器可以看成一个，切割过程的方法，把过程分割成不同状态的分割连接器，yield分割和传递产出，resume是clk激励信号
*/

#![allow(unused_imports)]
#![feature(generators, generator_trait)]
// #![feature(never_type)]
#![feature(negative_impls)]
#![feature(gen_future)]
use std::pin::Pin;
use std::ops::{Generator, GeneratorState};
/*
pub enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}

pub trait Generator<R = ()> {
    type Yield;
    type Return;
    fn resume(
        self: Pin<&mut Self>,
        arg: R
    ) -> GeneratorState<Self::Yield, Self::Return>;
}

#[unstable(feature = "generator_trait", issue = "43122")]
impl<G: ?Sized + Generator<R>, R> Generator<R> for Pin<&mut G> {
    type Yield = G::Yield;
    type Return = G::Return;

    fn resume(mut self: Pin<&mut Self>, arg: R) -> GeneratorState<Self::Yield, Self::Return> {
        G::resume((*self).as_mut(), arg)
    }
}

#[unstable(feature = "generator_trait", issue = "43122")]
impl<G: ?Sized + Generator<R> + Unpin, R> Generator<R> for &mut G {
    type Yield = G::Yield;
    type Return = G::Return;

    fn resume(mut self: Pin<&mut Self>, arg: R) -> GeneratorState<Self::Yield, Self::Return> {
        G::resume(Pin::new(&mut *self), arg)
    }
}
*/

fn main() {
    // book demo
    dbg!("book demo");
    let mut gen = || {
        let mut i = 1;
        while i < 10 {
            yield i;
            // yield i-1;
            // yield ();
            i += 1;
        }
        // return ();
        ()
    };

    loop {
        match Pin::new(&mut gen).resume(()) {
            GeneratorState::Yielded(y) => println!("Yielded: {:?}", y),
            GeneratorState::Complete(r) => {
                println!("Complete: {:?}", r);
                break; // if not break, will panic
                // thread 'main' panicked at 'generator resumed after completion', async_in_rust_33_generator.rs:22:19
            }
        }
    }

    // my simple demo
    dbg!("my simple demo");
    let mut g = || {
        println!("1: begin of generator");
        yield 2;
        // yield 'b';
        println!("3: after yield:\t 2");
        yield 4;
        // yield 'd';
        println!("5: after yield:\t 4");
        yield 6;
        // yield 'f';
        ()
    };

    let mut loop_cnt = 1;
    let mut resume_cnt = 1;
    loop {
        println!("loop counter:\t {:?}", loop_cnt); loop_cnt += 1;
        match Pin::new(&mut g).resume(()) {
            GeneratorState::Yielded(y) => {
                println!("Resume counter:\t {:?}", resume_cnt); resume_cnt += 1;
                println!("{y}: Rcv Yielded:\t {:?}", y);
            }
            GeneratorState::Complete(r) => {
                println!("Resume counter:\t {:?}", resume_cnt); resume_cnt += 1;
                println!("Complete: {:?}", r);
                break;
            }
        }
    }


    // official doc demo
    dbg!("official doc demo");
    let mut generator = || {
        println!("foo");
        yield 1;
        // return // ()
        // return; // ()
        // return 0; // int
        // return true; // bool
        "bar"
    };
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(1) => {}
        _ => panic!("Unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete("bar") => {}
        _ => panic!("Unexpected return from resume"),
    }


    // create g for self defined type
    dbg!("create g for self defined type");
    struct MyGenType {
        i: i32,
        completed: bool,
    }
    impl<R> Generator<R> for MyGenType {
        type Yield = i32;
        type Return = ();
        // type Return = u8;
        fn resume(self: Pin<&mut Self>, _arg: R) -> GeneratorState<Self::Yield, Self::Return> {
            if self.completed {
                panic!("MyGenType has been completed");
            }
            let i = self.i;
            if i < 10 {
                self.get_mut().i = i + 1;
                GeneratorState::Yielded(i)
            } else {
                self.get_mut().completed = true;
                GeneratorState::Complete(())
                // GeneratorState::Complete(255)
            }
        }
    }

    let mut mygen = MyGenType { i: 1, completed: false };
    loop {
        match Pin::new(&mut mygen).resume(()) {
            GeneratorState::Yielded(y) => println!("Yielded: {}", y),
            GeneratorState::Complete(r) => {
                println!("Complete: {:?}", r);
                break;
            }
        }
    }

    // iterator
    dbg!("iterator");
    use std::iter::Iterator;

    impl Iterator for MyGenType {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            match Pin::new(self).resume(()) {
                GeneratorState::Yielded(y) => Some(y),
                GeneratorState::Complete(()) => None,
            }
        }
    }
    let gen_iter = MyGenType { i: 1, completed: false };
    for val in gen_iter { println!("{}", val); }

    // change Yielded type and Return type
/*
    dbg!("change Yielded type and Return type(not work)");
    struct MyGenType2 {
        i: u8,
        completed: bool,
    }
    impl<R: std::cmp::PartialEq<()>> Generator<R> for MyGenType2 {
        type Yield = u8;
        type Return = bool;
        fn resume(self: Pin<&mut Self>, arg: R, in: u8) -> GeneratorState<Self::Yield, Self::Return> {
            match in {
              in if in != 0 => GeneratorState::Yielded(in),
              in if in == 255 => GeneratorState::Complete(true),
              _ => GeneratorState::Yielded(in),
            }
        }
    }

    let mut generator_customized = MyGenType2 { i: 0, completed: false };
    match Pin::new(&mut generator_customized).resume((), 1) {
        GeneratorState::Yielded(y) => { println!("{}", y); },
        // GeneratorState::Complete(y) => { println!("{}", y); },
        _ => { println!("Unexpected return from resume") },
    }
    let mut generator_customized = || {
        yield 1;
        yield 2;
        yield 3;
        yield 4;
        ()
    };

    loop {
        match Pin::new(&mut generator_customized).resume(()) {
            GeneratorState::Yielded(y) => println!("Yielded: {:?}", y),
            GeneratorState::Complete(r) => { println!("Complete ret: {:?}", r); break; },
        }
    }
 */

// The Rust Unstable Book
// generators
// The tracking issue for this feature is: #43122
// https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html
    dbg!("generator from unstable book");
    let mut generator_unstable_book = || {
        yield 1;
        // return "foo";
        // return "foo"
        "foo"
    };
    match Pin::new(&mut generator_unstable_book).resume(()) {
        GeneratorState::Yielded(1) => {},
        _ => panic!("Unexpected value from resume"),
    }
    match Pin::new(&mut generator_unstable_book).resume(()) {
        GeneratorState::Complete("foo") => {},
        _ => panic!("Unexpected value from resume"),
    }

    let mut generator_unstable_book_flow = || {
        println!("2");
        yield;
        println!("4");
    };
    println!("1");
    Pin::new(&mut generator_unstable_book_flow).resume(());
    println!("3");
    Pin::new(&mut generator_unstable_book_flow).resume(());
    println!("5");

    // capture var
    dbg!("generator capture environment variables");
    let x = 100;
    let y = 255;
    // let mut g_capture_1 = || {
    let mut g_capture_1 = move || {
        println!("1 {}", x);
        yield x;
        println!("2 {}", y);
        ()
    };
    println!("x: {}, y: {}", x, y);
    println!("{:?}", Pin::new(&mut g_capture_1).resume(()));
    println!("{:?}", Pin::new(&mut g_capture_1).resume(()));

    // Generators as state machines
    dbg!("Generators as state machines");
    let ret = "foo";
    let mut g_sm1 = move || {
        yield 1;
        return ret
    };
    // Pin::new(&mut g_sm1).resume(());
    println!("{:?}", Pin::new(&mut g_sm1).resume(()));
    println!("{:?}", Pin::new(&mut g_sm1).resume(()));

    // desugar
    dbg!("desugar");
/*
 */
    // #![feature(arbitrary_self_types, generators, generator_trait)]

    // use std::ops::{Generator, GeneratorState};
    // use std::pin::Pin;

    // fn main() {
        let ret = "foo";
        let mut generator = {
            enum __Generator {
                Start(&'static str),
                Yield1(&'static str),
                Done,
            }

            impl Generator for __Generator {
                type Yield = i32;
                type Return = &'static str;

                fn resume(mut self: Pin<&mut Self>, resume: ())
                    -> GeneratorState<i32, &'static str> {
                    use std::mem;
                    match mem::replace(&mut *self, __Generator::Done) {
                        __Generator::Start(s) => {
                            *self = __Generator::Yield1(s);
                            GeneratorState::Yielded(1)
                        }

                        __Generator::Yield1(s) => {
                            *self = __Generator::Done;
                            GeneratorState::Complete(s)
                        }

                        __Generator::Done => {
                            panic!("generator resumed after completion")
                        }
                    }
                }
            }

            __Generator::Start(ret)
        };

        // Pin::new(&mut generator).resume(());
        // Pin::new(&mut generator).resume(());
        println!("{:?}", Pin::new(&mut generator).resume(()));
        println!("{:?}", Pin::new(&mut generator).resume(()));
    // }

    // From Generator to Future
    /*
    dbg!("From Generator to Future");
    use std::future::ResumeTy;
    use std::task::{Context, Poll};
    use std::ptr::NonNull;
    use std::future::Future;

    pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
        where T: Generator<ResumeTy, Yield = ()>
    {
        struct GenFuture<T: Generator<ResumeTy, Yield = ()>>(T);
        impl<T: Generator<ResumeTy, Yield = ()>> !Unpin for GenFuture<T> {}

        impl<T: Generator<ResumeTy, Yield = ()>> Future for GenFuture<T> {
            type Output = T::Return;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let gen = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.0) };
                match gen.resume(ResumeTy(NonNull::from(cx).cast::<Context<'static>>())) {
                    GeneratorState::Yielded(()) => Poll::Pending,
                    GeneratorState::Complete(x) => Poll::Ready(x),
                }
            }
        }
        GenFuture(gen)
    }


    #[inline(never)]
    async fn foo() -> i32 {
        10
    }

    #[inline(never)]
    async fn bar() -> i32 {
        foo().await
    }

    #[inline(never)]
    async fn foo() -> impl Future<Output = i32> {
        from_generator(move |mut _task_context| {
            let _t = 10;
            _t
        })
    }

    #[inline(never)]
    async fn bar() -> impl Future<Output = i32> {
        from_generator(move |mut _task_context| {
            let _t = {
                match into_future(foo()) {
                    mut pinned => {
                        loop {
                            match unsafe Pin::new_unchecked(&mut pinned)
                                .poll(get_context(_task_context)) {
                                Poll::Ready(result) => break result,
                                Poll::Pending => {}
                            }
                            _task_context = (yield ());
                        }
                    }
                }
            };
            _t
        })
    }
     */
    // loop yeild
    dbg!("loop yeild");
    let mut g_loop = || { yield };
    loop {
        match Pin::new(&mut g_loop).resume(()) {
            GeneratorState::Yielded(y) => println!("Yielded: {:?}", y),
            GeneratorState::Complete(r) => { println!("Complete: {:?}", r); break },
        }
    }
}

