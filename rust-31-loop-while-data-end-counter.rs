fn main() {
// rust - loop while model
// do: do work
// break: condition to break loop
// cnt: counter
// start/end: init cnt and stop cnt
// n: loop iteration times
// while = loop { break - do/cnt }
//----------------------------------------
// 1: do - break - cnt - start - end - n
let start = 1;
let end   = 3;
let mut cnt = start;
loop {
    // do
    if cnt == end { break; }
    cnt += 1;
}
//----------------------------------------
// 2: do - cnt - break - start - end+1 - n
let start = 1;
let end   = 3;
let mut cnt = start;
loop {
    // do
    cnt += 1;
    if cnt == end+1 { break; }
}
//----------------------------------------
// 3: break - do - cnt - start - end+1 - n+1
let start = 1;
let end   = 3;
let mut cnt = start;
loop {
    if cnt == end+1 { break; }
    // do
    cnt += 1;
}
//----------------------------------------
// 4: break - cnt - do - start - end+1 - n+1
let start = 1;
let end   = 3;
let mut cnt = start;
loop {
    if cnt == end+1 { break; }
    cnt += 1;
    // do
}
//----------------------------------------
// 5: cnt - break - do - start - end+2 - n+1
let start = 1;
let end   = 3;
let mut cnt = start;
loop {
    cnt += 1;
    if cnt == end+2 { break; }
    // do
}
//----------------------------------------
// 6: cnt - do - break - start - end+1 - n
let start = 1;
let end   = 3;
let mut cnt = start;
loop {
    cnt += 1;
    // do
    if cnt == end+1 { break; }
}
//----------------------------------------
// while = loop { break - do/cnt } end n
// while pre-check condition before loop
// while cnt <= end {
  // do or cnt;
// }
//----------------------------------------
    println!("1-3-use-end-cnt");
    let mut i = 1;
    let mut loop_cnt = 0;
    loop {
        loop_cnt += 1;
        println!("loop cnt: {}, loop_in: {}", loop_cnt, i);
        // use data
        println!("use i: {}", i);
        // end condition
        if i == 3 {
            break;
        }
        // counter
        i += 1;
    }
/*
1-3-use-end-cnt
loop cnt: 1, loop_in: 1
use i: 1
loop cnt: 2, loop_in: 2
use i: 2
loop cnt: 3, loop_in: 3
use i: 3
*/
    println!("1-4-use-cnt-end");
    let mut i = 1;
    let mut loop_cnt = 0;
    loop {
        loop_cnt += 1;
        println!("loop cnt: {}, loop_in: {}", loop_cnt, i);
        // use data
        println!("use i: {}", i);
        // counter
        i += 1;
        // end condition
        if i == 4 {
            break;
        }
    }
/*
1-4-use-cnt-end
loop cnt: 1, loop_in: 1
use i: 1
loop cnt: 2, loop_in: 2
use i: 2
loop cnt: 3, loop_in: 3
use i: 3
*/
    println!("1-4-end-use-cnt-one-more-iter");
    let mut i = 1;
    let mut loop_cnt = 0;
    loop {
        loop_cnt += 1;
        println!("loop cnt: {}, loop_in: {}", loop_cnt, i);
        // end condition
        if i == 4 {
            break;
        }
        // use data
        println!("use i: {}", i);
        // counter
        i += 1;
    }
/*
1-4-end-use-cnt-one-more-iter
loop cnt: 1, loop_in: 1
use i: 1
loop cnt: 2, loop_in: 2
use i: 2
loop cnt: 3, loop_in: 3
use i: 3
loop cnt: 4, loop_in: 4
*/

    println!("0-3-end-cnt-use-one-more-iter");
    let mut i = 0;
    let mut loop_cnt = 0;
    loop {
        loop_cnt += 1;
        println!("loop cnt: {}, loop_in: {}", loop_cnt, i);
        // end condition
        if i == 3 {
            break;
        }
        // counter
        i += 1;
        // use data
        println!("use i: {}", i);
    }
/*
0-3-end-cnt-use-one-more-iter
loop cnt: 1, loop_in: 0
use i: 1
loop cnt: 2, loop_in: 1
use i: 2
loop cnt: 3, loop_in: 2
use i: 3
loop cnt: 4, loop_in: 3:w
*/

    println!("while-1-3-end-use-counter");
    let mut i = 1;
    let mut loop_cnt = 0;
    while i <= 3 {
        loop_cnt += 1;
        println!("loop cnt: {}, loop_in: {}", loop_cnt, i);
        println!("use i: {}", i);
        i += 1;
    }
/*
while-1-3-end-use-counter-one-more-iter
loop cnt: 1, loop_in: 1
use i: 1
loop cnt: 2, loop_in: 2
use i: 2
loop cnt: 3, loop_in: 3
use i: 3
*/
    println!("while-0-2-end-counter-use");
    let mut i = 0;
    let mut loop_cnt = 0;
    while i <= 2 {
        loop_cnt += 1;
        println!("loop cnt: {}, loop_in: {}", loop_cnt, i);
        i += 1;
        println!("use i: {}", i);
    }
/*
while-0-2-end-counter-use-one-more-iter
loop cnt: 1, loop_in: 0
use i: 1
loop cnt: 2, loop_in: 1
use i: 2
loop cnt: 3, loop_in: 2
use i: 3
*/
}
