#[derive(PartialEq, PartialOrd, Debug)]
enum Floor {
    ClientServices,
    Marketing,
    Ops,
}

fn is_below(this: &Floor, other: &Floor) -> bool {
    this < other
}
//demo2
#[derive(PartialEq, PartialOrd, Debug)]
enum Tax {
    Flat(f64),
    None,
    Percentage(f64),
}
fn smallest_amount(tax: Tax, other: Tax) -> Tax {
    if tax < other {
        tax
    } else {
        other
    }
}
fn main() {
    let first = Floor::ClientServices;
    let second = Floor::Marketing;
    if first == second {
        println!("{first:?} and {second:?} are in the same floor");
    } else {
        println!("{first:?} and {second:?} are not in the same floor");
    }

    if is_below(&first, &second) {
        println!("{first:?} floor is below {second:?}");
    } else {
        println!("{first:?} floor is not below {second:?}");
    }

    // demo2
    let no_tax = Tax::None;
    let flat_tax = Tax::Flat(5.5);
    println!("{:?}", smallest_amount(no_tax, flat_tax));

    let low = Tax::Flat(5.5);
    let high = Tax::Flat(8.0);
    println!("{:?}", smallest_amount(low, high));
}
