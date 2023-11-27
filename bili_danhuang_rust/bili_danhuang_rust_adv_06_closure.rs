// [RUST虐菜篇#06 | 小试闭包_哔哩哔哩_bilibili](https://www.bilibili.com/video/BV1oK41157Si?vd_source=91df0efcd22561d81b3a3a1b9f1299d0)
// sort algorithm, o(n) move data
fn mysort<T, U>(arr: &mut Vec<T>, op: U)
    where U: Fn(&T, &T) -> bool,
          T: std::fmt::Display + Copy
{
    dbg!(arr.len());
    if arr.len() <= 1 { return }
    // let mut iter_counter = 0;
    for i in 0..arr.len() {
        /*
        for j in i+1..arr.len() {
            println!("iteration counter: {iter_counter}, arr[{i}] {} cmp arr[{j}] {} result: {}", arr[i], arr[j], op(&arr[i], &arr[j]));
            iter_counter += 1;
            if op(&arr[i], &arr[j]) {
                // std::mem::swap(&mut arr[i], &mut arr[j]);
                let mut t = arr[i];
                arr[i] = arr[j];
                arr[j] = t;
            }
            // println!("iteration counter: {iter_counter}, arr[{i}] {} cmp arr[{j}] {} result: {}", arr[i], arr[j], op(&arr[i], &arr[j]));
        }
         */
        /*
         */
        let mut idx = i;
        for j in i+1..arr.len() {
            if !op(&arr[j], &arr[idx]) { idx = j; }
            // println!("iteration counter: {iter_counter}, arr[{i}] {} cmp arr[{idx}] {} result: {}", 
                // arr[i], arr[j], op(&arr[i], &arr[j]));
            // iter_counter += 1;
        }

        if idx == i { continue; }
        let mut d = arr[i];
        arr[i] = arr[idx];
        arr[idx] = d;
        // println!("iteration counter: {iter_counter}, arr[{i}] {} cmp arr[{idx}] {} result: {}", 
                // arr[i], arr[idx], op(&arr[i], &arr[idx]));
        // iter_counter += 1;
    }
}


fn main() {
    let mut myvec = vec![2, 8, 9, 5, 2, 3, 4, 1, 7, 0, 6];
    println!("{:?}", myvec);
    let cmp = | x: &i32, y: &i32 | x > y;
    mysort(&mut myvec, cmp);
    println!("{:?}", myvec);
}