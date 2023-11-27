// Chapter 6 - Text 
// https://tourofrust.com/chapter_6_en.html
/*
String Literals
What is utf-8
Escaping Characters
Multi-line String Literals
Raw String Literals
String Literals From Files
String Slice
Chars
String
Text As Function Parameters
Building Strings
Formatting Strings
Converting Strings
Chapter 6 - Conclusion
*/
/*
- String Literals
- String literals are always Unicode.
- String literals type are &'static str:
- & meaning that it's referring to a place in memory, and it lacks a &mut meaning that the compiler will not allow modification
- 'static meaning the string data will be available till the end of our program (it never drops)
- str means that it points to a sequence of bytes that are always valid utf-8
- Memory details:
- The Rust compiler will likely put your string in the data segment of your program memory
 */
fn str_lit() {
    dbg!("-- String Literals --");
    let a: &'static str = "hi 🦀";
    println!("{} {}", a, a.len());
}

/*
What is utf-8
As more languages were used on computers, the world needed to represent more text characters than ASCII allowed (1 byte only allowed 256 characters).
utf-8 was introduced with a variable byte length of 1-4 bytes greatly increasing the range of possible characters.
An advantage of variable sized characters is text did not have unnecessary bytes for very common ASCII (only requiring 1 byte still in utf-8).
A downside of variable sized characters is that character lookup can no longer be done quickly (O(1) constant time) with a simple indexing (e.g. my_text[3] to get the 4th character). It's possible that the preceding characters could have variable widths, altering where the 4th character actually begins in the sequence of bytes.
Instead we must iterate through a utf-8 byte sequence to understand where the Unicode characters actually begin (O(n) linear time).
Ferris: "I'm mostly just happy to have utf-8 for representing emojis of my underwater friends."

🐠🐙🐟🐬🐋
*/

/*
Escaping Characters
It's challenging to visually represent certain characters, so escape codes allow us to put a symbol in their place.
Rust supports the common escape codes from C-based languages:
\n - newline
\r - carriage return
\t - tab
\\ - backslash
\0 - null
\' - single-quote
The complete list exists here. https://doc.rust-lang.org/reference/tokens.html
\"	Double quote
\u{7FFF}	24-bit Unicode character code (up to 6 digits)
*/

fn escape_char() {
    dbg!("-- Escaping Chars --");
    let a: &'static str = "Ferris says: tab=\t,double quote=\"hello\", single quote=\', null=\0, blackslash=\\,carriage return=\r";
    println!("{}", a);
}

/*
Multi-line String Literals
Rust strings are multiline by default.
Use a \ at the end of a line if you don't want a line break.
*/
fn multi_line_str_literals() {
    dbg!("-- Multi-line String literals --");
    let haiku: &'static str = "
    I write, erase, rewrite
    Erase again, and then
    A poppy blooms.
    - Katsushika Hokusai";
    println!("{}", haiku);
    println!("Hello \
    world");
}

/*
Raw String Literals
Raw strings allow us to write a sequence of characters verbatim by starting with r#" and ending with "#. It lets us insert characters that might otherwise confuse a normal string as literals (like double quotes and backslashes).
*/
fn raw_str_literals() {
    dbg!("-- Raw String literals --");
    let a: &'static str = r#"<div class="advice">
        Raw strings are useful for some situations.
</div>
    "#;
    println!("{}", a);
}

/*
String Literals From Files
If you have some very large text, consider using the macro include_str! to include text from local files in your program:
let hello_html = include_str!("hello.html");
*/

/*
String Slice
A string slice is a reference to a sequence of bytes in memory that must always be valid utf-8.
A string slice (a sub-slice) of a str slice, must also be valid utf-8.
Common methods of &str:
len gets the length of the string literal in bytes (not number of characters).
starts_with/ends_with for basic testing.
is_empty returns true if zero length.
find returns an Option<usize> of the first position of some text.
*/
fn str_slice() {
    dbg!("-- String Slice --");
    let a = "hi 🦀"; //h=0,i=1,_=2,char=3,4,5,6
    println!("{}", a.len());
    let first_word = &a[0..2]; // 0..=1
    let second_word = &a[3..=6]; // 3..=6
    // let half_crab = &a[3..5]; // fails, // Rust does not accept slices of invalid unicode characters
    println!("{}\n{}", first_word, second_word);
}

/*
Chars
With so much difficulty in working with Unicode, Rust offers a way to retrieve a sequence of utf-8 bytes as a vector of characters of type char.
A char is always 4 bytes long (allowing for efficient lookup of individual characters).
*/
fn chars() {
    dbg!("-- Chars --");
    // collect the characters as a vector of char
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // 4
    // since chars are 4 bytes we can convert to u32
    println!("{:#010x}", chars[3] as u32);
    // https://magiclen.org/rust-formatted-string/
    // { [argument] ':' [[fill] align] [sign] ['#'] [width [$]] ['.' precision [$]] [type] }
    println!("Hello");                                     // => "Hello"
    println!("Hello, {}!", "world");                       // => "Hello, world!"
    println!("The number is {}", 1);                       // => "The number is 1"
    println!("{:?}", (3, 4));                              // => "(3, 4)"
    println!("{1:?} {0:?}", (3, 4), (5, 6));               // => "(5, 6) (3, 4)"
    println!("{value}", value = 4);                        // => "4"
    println!("{value1} {value2}", value1 = 4, value2 = 5); // => "4 5"
    println!("{name} {}", 1, name = "MagicLen");           // => "MagicLen 1"
    println!("{} {}", 1, 2);                               // => "1 2"
    println!("{0} {1}", 1, 2);                             // => "1 2"
    println!("{1} {0}", 1, 2);                             // => "2 1"
    println!("{1} {} {2} {} {0} {}", 1, 2, 3);             // => "2 1 3 2 1 3"
    println!("{:4}", 42);                                  // => "  42" with leading spaces
    println!("{:<4}", 42);                                 // => "42  " with trailing spaces
    println!("{:04}", 42);                                 // => "0042" with leading zeros
    println!("{:0<4}", 42);                                // => "4200" with trailing zeros
    println!("{:+}", 42);                                  // => "+42"
    println!("{:b}", 42);                                  // => "101010"
    println!("{:#b}", 42);                                 // => "0b101010"
    println!("{:.*}", 2, 1.2345);                          // => "1.23"
    println!("{:.*}", 3, 1.2345);                          // => "1.234"
    println!("{:.*}", 3, 1.2335);                          // => "1.234"
    println!("{:.3}", 1.2335);                             // => "1.234"
    println!("{:7.3}", 1.2335);                            // => "  1.234" with leading spaces
    println!("{:<7.3}", 1.2335);                           // => "1.234  " with trailing spaces
    println!("{:07.3}", 1.2335);                           // => "001.234" with leading zeros
    println!("{:0>7.3}", 1.2335);                          // => "001.234" with leading zeros
    println!("{:<07.3}", 1.2335);                          // => "001.234" with leading zeros
    println!("{:0<7.3}", 1.2335);                          // => "1.23400" with trailing zeros
    println!("{:07.3}", -1.2335);                          // => "-01.234" with leading spaces after the sign character
    println!("{:0>7.3}", -1.2335);                         // => "0-1.234" with leading spaces before the sign character
    println!("{:<07.3}", -1.2335);                         // => "-01.234" with leading zeros after the sign character
    println!("{:0<7.3}", -1.2335);                         // => "-1.2340" with trailing zeros
    println!("{:@>5}", "Hi");                              // => "@@@Hi" with leading '@'s
    println!("{:@<5}", "Hi");                              // => "Hi@@@" with trailing '@'s
    println!("{:@^5}", "Hi");                              // => "@Hi@@" with leading and trailing '@'s (center alignment)
    println!("{3:.2$} {1:.0$}", 2, 1.2345, 1, 1.2345);     // => "1.2 1.23"
    println!("{second:.second_decimal$} {first:.first_decimal$}", first_decimal = 2, first = 1.2345, second_decimal = 1, second = 1.2345); // => "1.2 1.23"
}

/*
String
A String is a struct that owns a sequence of utf-8 bytes in heap memory.
Because its memory is on the heap, it can be extended, modified, etc. in ways string literals cannot.
Common methods:
push_str to add more utf-8 bytes to the end of a string.
replace to replace sequences of utf-8 bytes with others.
to_lowercase/to_uppercase for case changes.
trim for trimming space
When a String is dropped, its heap memory is also dropped.
String has a + operator that extends the string with a &str and returns itself, but it might not be as ergonomic as you hope for.
*/
fn string() {
    dbg!("-- string --");
    let mut helloworld = String::from("hello");
    helloworld.push_str(" world");
    helloworld = helloworld + "!";
    println!("{}", helloworld);
}

/*
Text As Function Parameters
String literals and strings are generally passed around as a string slice to functions. This offers a lot of flexibility for most scenarios where you don't actually have to pass ownership.
*/
fn say_it_loud(msg: &str) {
    println!("{}!!!", msg.to_string().to_uppercase());
}
fn say_it_loud2(msg: String) {
    println!("{}!!!", msg.to_uppercase());
}
fn text_as_func_param() {
    dbg!("-- text as func parameters--");
    say_it_loud("hello");
    say_it_loud(&String::from("goodbye"));
    say_it_loud2(String::from("goodbye"));
}

/*
Building Strings
concat and join are two simple but powerful ways for building strings.
*/
fn building_strings() {
    dbg!("-- building Strings");
    // pub fn concat<Item>(&self) -> <[T] as Concat<Item>>::Outputⓘ
    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}", abc);
}

/*
Formatting Strings
The format! macro allows us to create a string by defining a parameterized string with placeholders for where and how values should be placed (e.g. {}).
format! uses the same parameterized strings as println!
The capabilities of this function are too large of scope for Tour of Rust; check out the documentation here.
*/
fn formatting_strings() {
    dbg!("--formatting String --");
    let a = 42;
    let f = format!("secret to life: {}", a);
    println!("{}", f);
}

/*
Converting Strings
Many types can be converted to a string using to_string.
The generic function parse can be used to convert strings or string literals into a typed value. This function returns a Result because it could fail.
*/
fn convert_strings(text: String) -> Result<(), std::num::ParseIntError>{
    dbg!("-- Converting Strings");
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    let c = text.parse::<i32>()?;
    println!("{} {} {}", a, b, c);
    Ok(())
}

fn main() {
    str_lit();
    escape_char();
    multi_line_str_literals();
    raw_str_literals();
    str_slice();
    chars();
    string();
    text_as_func_param();
    building_strings();
    formatting_strings();
    convert_strings("100".to_string());
    match convert_strings("".to_string()) {
        Ok(()) => {},
        Err(e) => println!("{:?}", e),
    }

}

/*
Chapter 6 - Conclusion
Now you know the basics of text! As you have seen, Unicode makes working with text a bit tricky, but the standard library has plenty of functionality to make it easy to manage.
Up to now, we've mostly looked at Rust from the lens of a procedural paradigm (i.e. just functions and data), but it's time we now talk about traits and the capabilities unlocked by Rust's object oriented paradigm.
*/