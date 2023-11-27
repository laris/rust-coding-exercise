fn main() {
    let _v: () = ();
    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());
    assert_eq!(_v, explicitly_ret_unit());
    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

fn explicitly_ret_unit() -> () {
    //println!("I will return a ()")
    println!("I will return a ()");
    //()
}
