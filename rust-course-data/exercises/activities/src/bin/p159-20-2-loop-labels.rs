//'ident: loop {}
//'ident: for x in y {}
//'ident: while true {}

fn main() {
    println!("----------break label----------");
    let matrix = [
        [ 2,  4,  6],
        [ 8,  9, 10],
        [12, 14, 16],
    ];
    'rows: for row in matrix.iter() {
        println!("row: {row:?}");
        'cols: for col in row {
            println!("col: {col:?}");
            if col % 2 == 1 {
                println!("odd: {}", col);
                break 'rows;
            }
            println!("{col}");
        }
    }

    println!("----------continue label----------");
    let matrix = [
        [ 2,  4,  6],
        [ 8,  9, 10],
        [12, 14, 16],
    ];
    'rows: for row in matrix.iter() {
        println!("row: {row:?}");
        'cols: for col in row {
            println!("col: {col:?}");
            if col % 2 == 1 {
                println!("odd: {}", col);
                continue 'rows;
            }
            println!("{col}");
        }
    }
    println!("----------continue----------");
    type UserInput<'a> = Result<&'a str, String>;
    'menu: loop {
        println!("menu");
        'input: loop {
            let user_input: UserInput = Ok("next");
            //let user_input: UserInput = Err("error".into());
            match user_input {
                Ok(input) => break 'menu,
                Err(_) => {
                    println!("try again");
                    continue 'input;
                }
            }
        }
    }
}
