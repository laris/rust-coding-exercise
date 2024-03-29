macro_rules! m {
    (==>)   => { print!("1"); };
    (= = >) => { print!("2"); };
    (== >)  => { print!("3"); };
    (= =>)  => { print!("4"); };
    (<<=)   => { print!("5"); };
    (<==)   => { print!("6"); };
    (< ==)  => { print!("7"); };
    (=<<)   => { print!("8"); };
    (= <<)  => { print!("9"); };
    (= < <) => { print!("a"); };
}

fn main() {
    m!(==>);    // == >  1
    m!(= = >);  // = = > 2
    m!( =  =  > );  // = = > 2
    m!(== >);   // == >  1
    m!(= =>);   // = =>  4
    m!(<<=);    // <<=   5
    //m!(< < =); // Err: <<= need consecutively without spaces
    m!(<==);    // <==   6
    m!(< ==);   // < ==  7
    m!(=<<);    // = <<  8
    m!(= <<);   // = <<  8
    m!(= < <);  // = < < a
}
