fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];
	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}

fn largest2<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];
	for item in list.iter() {
		if *item > largest {
			largest = *item;
		}
	}
	largest
}

fn largest3<T: PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];
	for item in list.iter() {
		if item > largest {
			largest = item;
		}
	}
	largest
}

fn largest4<T: PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];
	for item in list.iter() {
		if *item > *largest {
			largest = item;
		}
	}
	largest
}

fn main() {
	let number_list = vec![34, 50, 25, 100, 65];

	let result = largest(&number_list);
	println!("The largest number is {}", result);
	let result = largest2(&number_list);
	println!("The largest number is {}", result);
	let result = largest3(&number_list);
	println!("The largest number is {}", result);
	let result = largest4(&number_list);
	println!("The largest number is {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest(&char_list);
	println!("The largest char is {}", result);
}