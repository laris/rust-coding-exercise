// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    match tile {
        Tile::Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red | brick @ BrickStyle::Dungeon) => println!("Brick style: {brick:?}"),
        Tile::Brick(other) => println!("other ({other:?}) brick style"),
        /*
        Tile::Brick(brick_stype) => {
            match brick_stype {
                BrickStyle::Dungeon => println!("{brick_stype:?}"),
                BrickStyle::Gray    => println!("{brick_stype:?}"),
                BrickStyle::Red     => println!("{brick_stype:?}"),
            }
        }
        */
        Tile::Dirt | Tile::Grass | Tile::Sand | Tile::Wood => println!("Ground Tile: {tile:?}"),
        //Tile::Dirt  => println!("{tile:?}"),
        //Tile::Grass => println!("{tile:?}"),
        //Tile::Sand  => println!("{tile:?}"),
        //Tile::Wood => println!("{tile:?}"),
        /*
        Tile::Treasure(treasure_chest) => {
            let TreasureChest { content, amount } = treasure_chest;
            match content {
                TreasureItem::Gold => println!("{content:?}"),
                TreasureItem::SuperPower=> println!("{content:?}"),
            }
            println!("amount: {amount:?}");
        }
        */
        Tile::Treasure(TreasureChest { amount, .. }) if amount >= 100 => println!("Lots of treasures, amount: {amount}"),
        Tile::Treasure(TreasureChest { content, .. }) => println!("Treasure content: {content:?}"),
        /*
        Tile::Water(pressure) => {
            let Pressure(p) = pressure;
            println!("Pressure: {pressure:?}");
        }
        */
        Tile::Water(pressure) if pressure.0 < 10 => println!("Water pressure level: {}", pressure.0),
        Tile::Water(pressure) if pressure.0 >= 10 => println!("High water pressure! level: {}", pressure.0),
        _ => println!("other tile no-exist"),
    }
}
fn main() {
    let tile = Tile::Brick(BrickStyle::Red);
    print_tile(tile);

    let tile = Tile::Sand;
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 200,
    });
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 90,
    });
    print_tile(tile);

   let tile = Tile::Water(Pressure(9));
   print_tile(tile);
}

// add test example
#[cfg(test)]
mod tests {
    use super::*;
    fn return_tile_string(tile: Tile) -> String {
        match tile {
            Tile::Water(pressure) if pressure.0  < 10 => format!("Water pressure level: {}", pressure.0),
            Tile::Water(pressure) if pressure.0 >= 10 => format!("High water pressure! level: {}", pressure.0),
            _ => format!("other tile"),
        }
    }
    #[test]
    fn test_water_lower() {
        let tile = Tile::Water(Pressure(0));
        assert_eq!(return_tile_string(tile), "Water pressure level: 0");
    }
}
