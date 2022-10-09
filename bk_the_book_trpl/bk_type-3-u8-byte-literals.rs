fn main() {

	let u8code65_fmt_cap_a = b'A';
	let u8code65_fmt_u8dec = 65u8;
	let u8code65_fmt_hex = b'\x41';

	println!("u8 value 65 with ASCII literals fmt (b'A') = {}", u8code65_fmt_cap_a);
	println!("u8 value 65 with dec u8 fmt (65u8) = {}", u8code65_fmt_u8dec);
	println!("u8 value 65 with hex literal fmt (b'\x41') = {}", u8code65_fmt_hex);

	/*
	for char_special in b'\''

	Character				Byteliteral		Numeric equivalent
	Single quote,	' b'\''					39u8
	Backslash, \		b'\\'					92u8
	Newline					b'\n'					10u8
	Carriagereturn	b'\r'					13u8
	Tab							b'\t'					9u8
	*/
}
