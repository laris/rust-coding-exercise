trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) { println!("checked in as pilot"); } 
    fn process(&self) { println!("pilot takes a seat"); } 
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) { println!("checked in as Passenger"); } 
    fn process(&self) { println!("Passenger takes a seat"); } 
}
struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) { println!("Cargo checked in"); } 
    fn process(&self) { println!("Cargo moved to storage"); } 
}

fn process_item<T: CheckIn>(item: T) {
    item.check_in();
    item.process();
}

fn main() {
    let paul = Passenger;
    let kathy = Pilot;
    let cargo1 = Cargo;
    let cargo2 = Cargo;
    process_item(paul);
    process_item(kathy);
    process_item(cargo1);
    process_item(cargo2);
}
