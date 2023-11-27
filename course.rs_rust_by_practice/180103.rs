/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
fn main() {
     let movable = Box::new(3);

     let consume = || {
         println!("`movable`: {:?}", movable);
         take(movable);
     };

     consume();
     consume();
}

fn take<T>(_v: T) {}
*/

fn main() {
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {movable:?}");
        //take(movable);
        take_ref(&movable);
    };
    consume();
    consume();

    let consume_move = move || { println!("movable: {movable:?}");};
    consume_move();

}
fn take<T>(_v: T) {}
fn take_ref<T>(_v: &T) {}

