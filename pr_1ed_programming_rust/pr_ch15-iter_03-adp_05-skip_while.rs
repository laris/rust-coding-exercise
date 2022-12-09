fn main() {
    /*
    fn skip(self, n: usize) -> impl Iterator<Item=Self::Item>
        where Self: Sized;
    fn skip_while(self, predicate: P) -> impl Iterator<Item=Self::Item>
        where Self: Sized, P: FnMut(&Self::Item) -> bool;
    */

    let message = "To: jimb\r\n\
                   From: superego <editor@oreilly.com>\r\n\
                   \r\n\
                   Did you get any writing done today?\r\n\
                   When will you stop wasting time plotting fractals?\r\n";
    for body in message.lines()
                        .skip_while(|l| !l.is_empty())
                        .skip(1) {
        println!("{}", body);
    }
}

