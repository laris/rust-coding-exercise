use std::rc::Rc;

#[derive(Debug)]
struct Vehicle {
    vin: String,
}
#[derive(Debug)]
struct Door {
    vehicle: Rc<Vehicle>,
}

fn main() {
    let car = Rc::new(Vehicle {
        vin: "123".into(),
    });

    let left_door = Door {
        vehicle: Rc::clone(&car),
    };
    let right_door = Door {
        vehicle: Rc::clone(&car),
    };
    drop(car);
    println!("Vehicle = {:?}", left_door.vehicle);
    println!("Vehicle = {:?}", right_door.vehicle);
}
