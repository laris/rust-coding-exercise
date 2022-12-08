fn main() {
    // map
    //fn map<B, F>(self, f: F) -> impl Iterator<Item=B>
    //   where Self: Sized, F: FnMut(Self::Item) -> B;
    let text = " ponies \n giraffes\niguanas \nsquid".to_string();
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        //.map(|s|str::trim(s))
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);

    // filter
    // fn filter<P>(self, predicate: P) -> impl Iterator<Item=Self::Item>
    //    where Self: Sized, P: FnMut(&self::Item) -> bool;
    let v: Vec<&str> = text.lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    // lazy test
    let mut i = ["earth", "water", "air", "fire"]
        .iter().map(|elt| println!("{:?}", elt));
    // note: iterators are lazy and do nothing unless consumed
    i.next();
    i.next();
    i.next();
    i.next();
    i.next(); // None
    i.next(); // None

    // zoro-overhead abstraction
    let mut v: Vec<&str> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line != "iguanas" {
            v.push(line);
        }
    }
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
}
