fn main() {
    let value = 5;
    let result: usize = 
        'ident: loop {
            break value;
            break 'ident value;
        };

    fn get_input<'a>() -> Result<&'a str, String> {
        Ok("0")
    }
    let value: usize = loop {
        if let Ok(input) = get_input() {
            match input.parse::<usize>() {
                Ok(value) => break value,
                Err(e) => continue,
            }
        }
    };

    let nums = vec![1, 2, 4, 5, 6, 7, 8];
    let div_by_three: Option<usize> = 'outer: loop {
        for n in nums {
            if n % 3 == 0 {
                break 'outer Some(n);
            }
        }
        break None;
    };
}
