struct S {
    x: i32,
}

const S: S = S { x: 2 };

fn main() {
    let v = &mut S;
    v.x += 1;
    S.x += 1;
    println!("{}{}", v.x, S.x);

    // list all possible left/right value with mut or not
    let     lva =  S;
    let     lvb = &S;

    let mut lvc =  S;
    let mut lvd = &S;
    struct R { y: f64, }
    const R: R = R { y: 3.1415926 };
    // Err: re-bind to another type
    //lvc = R;
    //lvd = &R;
    const S2: S = S { x: 10 };
    lvc =  S2;
    lvd = &S2;

    // let mut lve = mut S; // error: expected expression
    let mut lvf = &mut S;

    // let     lvg = mut S; // error
    let     lvh = &mut S;
}
