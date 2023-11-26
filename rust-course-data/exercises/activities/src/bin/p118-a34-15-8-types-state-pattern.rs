struct BusTicket;
struct BoardedBusTicket;

impl BusTicket {
    fn board(self) -> BoardedBusTicket {
        BoardedBusTicket
    }
}

struct File<'a>(&'a str);
impl<'a> File<'a> {
    fn read_bytes(&self) -> Vec<u8> {
        // ... read data ...
        println!("read_bytes(&self) ");
        Vec::new()
    }
    fn delete(self) {
        // ... delete file ...
        println!("delete file");
    }
}

fn main() {
    let ticket = BusTicket;
    let boarded = ticket.board();
    // compile error
    //ticket.board();

    let file = File("data.txt");
    let data = file.read_bytes();
    file.delete();
    // Compile error
    //let read_again = file.read_bytes();
}
