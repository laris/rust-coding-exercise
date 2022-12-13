fn main() {
    let packed  = "Helen of Troy";
    let spaced  = "Helen  of   Troy";
    let obscure = "Helen of Sandusky"; // nice person, just not famous

    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace())); // split_whitespace() return iterator

    // This is true because ' ' < 'o'
    assert!(spaced < obscure);

    // This is true because 'Troy' > 'Sandusky'.
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));
}
