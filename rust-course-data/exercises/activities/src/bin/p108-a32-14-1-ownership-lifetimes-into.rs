#[derive(Debug)]
enum FrozenItem { IceCube, }

#[derive(Debug)]
struct Freezer { contents: Vec<FrozenItem>, }

fn place_item(freezer: &mut Freezer, item: FrozenItem) {
    freezer.contents.push(item);
}
// lifetime
enum Part { Bolt, Panel, }
struct RobotArm<'a> { part: &'a Part, }
struct AssemblyLine { parts: Vec<Part>, }

// lifetime function
struct DataType;
fn name_with_lifetime<'a>(arg: &'a DataType) -> &'a DataType { &DataType }

fn main() {
    let mut freezer = Freezer { contents: vec![] };
    let cube = FrozenItem::IceCube;
    place_item(&mut freezer, cube);
    // cube no longer avaliable
    println!("{freezer:?}");
    // lifetime
    let line = AssemblyLine { parts: vec![Part::Bolt, Part::Panel], };
    { 
        let arm = RobotArm { part: &line.parts[0], };
    }
    // arm no longer exists
}
