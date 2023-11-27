use std::fmt;

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

impl fmt::Pointer for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let ptr = self as *const Self;
		fmt::Pointer::fmt(&ptr, f)
	}
}

#[derive(Clone)]
struct PointString {
	x: String,
}
impl fmt::Pointer for PointString {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let ptr = self as *const Self;
		fmt::Pointer::fmt(&ptr, f)
	}
}

#[derive(Clone)]
struct PB {
	x: Box<i32>,
	y: Box<i32>,
}
impl fmt::Pointer for PB {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let ptr = self as *const Self;
		fmt::Pointer::fmt(&ptr, f)
	}
}

fn main() {
	let pb1 = PB { x: Box::new(0), y: Box::new(0)};
	let pb2 = pb1.clone();
	println!("{pb1:p}");
	println!("{pb2:p}");
	println!();
	let ref pb @ PB { x: ref pb_x, y: ref pb_y, } = PB {x: Box::new(1), y: Box::new(1)};
	println!("{pb_x:p}, {pb_y:p}");
	println!("{pb:p}");
	println!();

	let ps1 = PointString { x: "".into(), };
	let ps2 = ps1.clone();
	println!("{ps1:p}");
	println!("{ps2:p}");
	// let ps @ PointString { x: ps_x } = PointString { x: "".into() };
	// println!("{ps:p}");
	// println!("{:p}", &ps_x);
	println!("");


	let p1 = Point { x: 1, y: 1 };
	let p2 = Point { x: 2, y: 1 };
	let p3 = Point { x: 3, y: 1 };
	println!("{:p}", &p1);
	println!("{:p}", &p2);
	println!("{p3:p}");

	// 绑定新变量 `p`，同时对 `Point` 进行解构
	let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
	println!("{:p}", &p);
	println!("x: {:p}, y: {:p}", &px, &py,);

	/*
	let int32 = 1;
	println!("{}", int32);
	println!("{:p}", &int32);
	println!("{:?}", &int32 as *const i32);
	println!("{:p}", &&int32);

	let int32_copy = int32;
	println!("{}", int32_copy);
	println!("{:p}", &int32_copy);
	println!("{:?}", &int32_copy as *const i32);
	println!("{:p}", &&int32_copy);
	*/

	let point = Point {x: 10, y: 5};
	println!("point's addr: {:p}", &point);
	if let p @ Point {x: 10, y} = point {
		println!("x is 10 and y is {:p} in {:p}", &y, &p);
	} else {
		println!("x was not 10 :(");
	}
}
