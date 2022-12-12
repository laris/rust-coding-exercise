/*
impl<'a I: Iterator + ?Sized> Iterator for &'a mut I {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}
*/
fn main() {
    let msg = "To: jimb\r\n\
               From: id\r\n\
               \r\n\
               Oooooh, donuts!!\r\n";
    let mut lines = msg.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("\nBody:");
    //for body in lines.by_ref() {
    for body in lines {
        println!("{}", body);
    }

    // if use iterator, that's lifetime will gone and we cannot borrow it
    //println!("{lines:?}");
}
