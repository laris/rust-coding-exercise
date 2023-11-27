fn main() {
    let c      = '中';
    let r1     = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));
}

fn get_addr(r: &char) -> String {
    format!("{r:p}")
}
