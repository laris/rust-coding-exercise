//#[deny(arithmetic_overflow)]
#[allow(arithmetic_overflow)]

fn main() {

	let big_val = std::i32::MAX;
	println!("{:?}", big_val);

	let x = big_val.wrapping_add(1);
	println!("{:?}", x); // work in release build

	/*
  	 #[deny(arithmetic_overflow)]  <-- default
     error: this arithmetic operation will overflow <-- build Err
     #[allow(arithmetic_overflow)] <-- can build but still panic when run
	   thread 'main' panicked at 'attempt to add with overflow', type-2-max-val-wrap.rs:11:13
	*/
	let big_val = std::i32::MAX;
	let x = big_val + 1; // work in debug build
	println!("{:?}", x);

}
