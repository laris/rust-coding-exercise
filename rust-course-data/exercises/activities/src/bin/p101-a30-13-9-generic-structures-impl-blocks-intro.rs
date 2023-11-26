trait Game {
    fn name(&self) -> String;
}
#[derive(Debug)]
enum BoardGame {
    Chess,
    Monopoly,
}

impl Game for BoardGame {
    fn name(&self) -> String { format!("{self:?}") }
}

#[derive(Debug)]
enum VideoGame {
    PlayStation,
    Xbox,
}

impl Game for VideoGame {
    fn name(&self) -> String { format!("{self:?}") }
}

#[derive(Debug)]
struct PlayRoom<T: Game> {
    game: T,
}

impl PlayRoom<BoardGame> {
    pub fn cleanup(&self) { println!("cleanup for {self:?}");}
}

impl PlayRoom<VideoGame> {
    pub fn cleanup(&self) { println!("cleanup for {self:?}");}
}

impl<T: Game> PlayRoom<T> {
    pub fn game_info(&self) {
        println!("{:?}", self.game.name());
    }
}

trait Trait1 {}
trait Trait2 {}
trait Trait3 {}

struct StructName<T: Trait1 + Trait2, U: Trait3> {
    field1: T, field2: U, }
impl<T: Trait1 + Trait2, U: Trait3> StructName<T, U> {
    fn fnc(&self, arg1: T, arg2: U) {}
}

struct StructName2<T, U>
where T: Trait1 + Trait2,
      U: Trait3,
{ field1: T, field2: U, }
impl<T, U> StructName2<T, U>
where T: Trait1 + Trait2,
      U: Trait3, 
{ fn fnc(&self, arg1: T, arg2: U) {} }


fn main() {
    println!("{:?}", BoardGame::Chess.name());
    println!("{:?}", VideoGame::PlayStation.name());

    let board_room = PlayRoom { game: BoardGame::Monopoly, };
    let video_room = PlayRoom { game: VideoGame::PlayStation, };
    board_room.cleanup();
    video_room.cleanup();
    video_room.game_info();
}

