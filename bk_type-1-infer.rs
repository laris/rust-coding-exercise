fn build_vector1() -> Vec<i16> {
	let mut v: Vec<i16> = Vec::<i16>::new();
	v.push(10i16);
	v.push(20i16);
	println!("{:?}", v);
	v
}

fn build_vector2() -> Vec<i16> {
	let mut v = Vec::new();
	v.push(10);
	v.push(20);
	println!("{:?}", v);
	v
}

fn main(){
	build_vector1();
	build_vector2();
}

