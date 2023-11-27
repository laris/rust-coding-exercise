#![allow(dead_code)]
#![allow(unused_variables)]
struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

fn main() {
    let a = A;
    let aa = A{};
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(0i32));
    generic::<char>(SGen('a'));
    generic(SGen('a'));
}
