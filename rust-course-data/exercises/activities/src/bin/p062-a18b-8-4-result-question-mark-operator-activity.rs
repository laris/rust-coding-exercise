// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

// * Use an enum to represent all types of employees
#[derive(Debug)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStuff,
    AssemblyTechnician,
}

#[derive(Debug)]
enum Status {
    Active,
    Terminated,
}
// * Use a struct to store the employee type and whether they are
//   still employed
#[derive(Debug)]
struct Employee {
    position: Position,
    status: Status,
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn try_access(emp: &Employee) -> Result<(), String> {
    //if emp.status { Ok(()) }
    //else { Err(format!("{:?} is fired", emp)) }
    match emp.status {
        Status::Terminated => return Err(format!("{:?} is terminated", emp)),
        _ => (),
    }
    match emp.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err(format!("Position: {:?} is not allowed to access", emp.position)),
    }
}
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access(emp: &Employee) -> Result<(), String> {
    let attempt_access = try_access(emp)?;
    println!("{:?} access ok", emp);
    Ok(())
}

fn main() {
    let jimmy = Employee { position: Position::Manager, status: Status::Active };
    println!("{:?}", try_access(&jimmy));
    match print_access(&jimmy) {
        Err(e) => println!("{:?}: Access denied, reason: {:?}", jimmy, e),
        Ok(()) => println!("ok"), 
    }

    let tommy = Employee { position: Position::KitchenStuff, status: Status::Active};
    println!("{:?}", try_access(&tommy));
    match print_access(&tommy) {
        Err(e) => println!("{:?}: Access denied, reason: {:?}", tommy, e),
        Ok(()) => println!("ok"), 
    }

    let john = Employee { position: Position::AssemblyTechnician, status: Status::Terminated };
    println!("{:?}", try_access(&john));
    match print_access(&john) {
        Err(e) => println!("{:?}: Access denied, reason: {:?}", john, e),
        Ok(()) => println!("ok"), 
    }
}
