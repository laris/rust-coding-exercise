fn main() {
    let byte_escape = "I'm writing Ru\x73\x74";
    println!("What are you doing\x3F (\\x3F mean ?) {byte_escape}");    

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("'Unicode character {unicode_codepoint} (U+211D) is called {character_name}");

    let long_string = "String literals
    can span multiple lines.
The linebreak and indentation here \
                        can be escaped too!";
    println!("{long_string}");
}
