fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }
}
