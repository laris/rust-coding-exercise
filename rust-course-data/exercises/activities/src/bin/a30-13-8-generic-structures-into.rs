trait Trait1 {}
trait Trait2 {}
trait Trait3 {}

struct StructName<T: Trait1 + Trait2, U: Trait3> {
    field1: T,
    field2: U,
}

struct StructName2<T, U>
where
    T: Trait1 + Trait2,
    U: Trait3,
{
    field1: T,
    field2: U,
}

trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    location: T,
}

#[derive(Debug, Clone, Copy)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    Back(u32),
}
impl Seat for ConcertSeat {
    fn show(&self) {
        println!("{self:?}");
    }
}
#[derive(Debug, Clone, Copy)]
enum AirlineSeat {
    BusinessClass,
    Economy,
    FirstClass,
}
impl Seat for AirlineSeat {
    fn show(&self) {
        println!("{self:?}");
    }
}

fn ticket_info(ticket: Ticket<AirlineSeat>) {
    ticket.location.show()
}
fn ticket_info2<T: Seat>(ticket: Ticket<T>) {
    ticket.location.show()
}
fn main() {
    ConcertSeat::Back(100).show();
    ConcertSeat::show(&ConcertSeat::Back(100));
    AirlineSeat::FirstClass.show();

    let airline = Ticket { location: AirlineSeat::FirstClass };
    let concert = Ticket { location: ConcertSeat::FrontRow };
    ticket_info(airline);
    ticket_info2(concert);
}
