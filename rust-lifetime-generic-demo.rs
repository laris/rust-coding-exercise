// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=85b8ed82a92b08c5b9b11380cd8d7549

fn _longest1<'a100>(x: &'a100 str, y: &'a100 str) -> &'a100 str {
    if x.len() > y.len() { x } // 'a100
    else { y }                 // 'a100
}

fn _longest21<'a100: 'b50, 'b50>(x: &'a100 str, y: &'b50 str) -> &'b50 str {
    if x.len() > y.len() { x }  // 'a100 <-- x输入参数定义，y满足'a100约束，
                                //  范型定义'a100: 'b50，满足'a100约束的必定满足'b50约束，所以可以做返回值，满足'b50
    else { y }                  // 'b50  <-- y输入参数定义，y满足'b50的约束，符合返回约束
}

// fn _longest22<'a100: 'b50, 'b50>(x: &'a100 str, y: &'b50 str) -> &'a100 str { // error: lifetime may not live long enough
//     // lifetime may not live long enough
//     if x.len() > y.len() { x }  // 'a100
//     else { y }                  // 'b50 <-- 返回的y不满足 'a100的约束
// }

fn _longest23<'a100: 'b50, 'b50: 'a100>(x: &'a100 str, y: &'b50 str) -> &'a100 str {
    // lifetime may not live long enough
    if x.len() > y.len() { x }  // 'a100
    else { y }                  // 'b50 <-- 返回的y满足 'a100的约束，因为<'a100: 'b50, 'b50: 'a100>
}

fn _longest31<'a: 'b, 'b>(x: &'a str, y: &'a str) -> &'b str {
    if x.len() > y.len() { x }  // x, y 都满足'a约束，两者没有区别，范型定义了'a: 'b，
    else { y }                  // 那就是说，满足'a的x, y必定满足'b，自然满足返回要求的'b
}

// fn _longest32<'a, 'b: 'a>(x: &'a str, y: &'a str) -> &'b str { // error: lifetime may not live long enough
//     if x.len() > y.len() { x }  // x, y 都满足'a约束，两者没有区别，范型定义了'b: 'a，
//     else { y }                  // 但是内部没有满足'b约束的变量做输出，换言之，就是xy都不满足'b的约束
// }

fn _longest33<'a, 'b: 'a, 'c: 'b>(x: &'a str, y: &'a str) -> &'b str {
    let ref z: &'c str = "xyz"; // 既然xy都不满足'b的约束，内部构造一个z，z的生命周期是'c，但是'c: 'b，就是'c一定满足'b，可以满足返回值要求
    if x.len() > y.len() { z }  // 直接返回z
    else { z }                  // 直接返回z

}

fn _longest4<'a>(x: &'a str, y: &'a str) -> &str {
    // Lifetime elision in functions https://doc.rust-lang.org/reference/lifetime-elision.html
    if x.len() > y.len() { x }  // If there is exactly one lifetime used in the parameters (elided or not),
    else { y }                  // that lifetime is assigned to all elided output lifetimes. 也就是输出也是'a
}


fn main() {
    // let ref x = "abc";
    // let ref y = "123";
    // println!("x = {x}, y = {y}, longest = {}", longest1(x, y));

}