trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Snake;
impl Move for Snake {
    fn move_to(&self, x: i32, y: i32) {
        println!("slither to ({},{})", x, y);
    }
}

struct Grasshopper;
impl Move for Grasshopper {
    fn move_to(&self, x: i32, y: i32) {
        println!("hop to ({}{})", x, y);
    }
}
// fn func(parm: impl Trait) {}
//fn function<T: Trait1, U: Trait2>(param1: T, param2: U) { /* body */ }
/*
fn function2<T, U>(param1: T, param2: U) 
where
    T: Trait1 + Trait2,
    U: Trait1 + Trait2 + Trait3,
{
    /* body */
}
*/

fn make_move(thing: impl Move, x: i32, y: i32) {
    thing.move_to(x, y);
}

fn make_move2<T: Move>(thing: T, x: i32, y: i32) { thing.move_to(x, y); }

fn make_move3<T>(thing: T, x: i32, y: i32)
    where T: Move,
{ thing.move_to(x, y); }

fn main() {
    let python = Snake {};
    make_move(python, 1, 1);
}
