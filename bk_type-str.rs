fn main() {

	let speech = "\"Ouch!\" said the well.\n";
	println!("{}", speech);

	let multi_lines = "In the room the women come and go,
Singing of Mount Abora.
 .
  .
   .
    ....
\t.";
	println!("{}", multi_lines);


	let line_backslash = "\
It was a bright, cold day in April, and \
there were four of us -- \
more or less.";
	println!("{}", line_backslash);

	let default_win_install_path1 = "C:\\Program Files\\Gorillas";
	println!("{}", default_win_install_path1);

	let default_win_install_path2 = r"C:\Program Files\Gorillas";
	println!("{}", default_win_install_path2);

//	let pattern = Regex::new(r"\d+(\.\d+)*");

	println!(r###"
			This raw string started with 'r###"'.
			Therefore it does not end until we reach a quote mark ("")
			followed immediately by three pound signs ('###'):
			"###);

}
