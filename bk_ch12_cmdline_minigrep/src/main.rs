
fn main() {
    // listing 12-1 collect cmdline args into vec and print
    /*
    use std::env;
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
     */

     // lst 12-2 borrow arg into &str
     /*
    use std::env;
     let args: Vec<String> = env::args().collect();
     let query = &args[1];
     let filename = &args[2];
     println!("Search for: {}\nIn file: {}", query, filename);
      */

    // lst 12-3 poem.txt
    /*
    I'm nobody! Who are you?
    Are you nobody, too?
    Then there's a pair of us — don't tell!
    They'd banish us, you know.

    How dreary to be somebody!
    How public, like a frog
    To tell your name the livelong day
    To an admiring bog!”

    Excerpt From
    The Rust Programming Language (Covers Rust 2018)
    Steve Klabnik
    This material may be protected by copyright. 
    */

    // lst 12-4 read from file
    /* 
    use std::env;
    use std::fs;

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    println!("Search for: {}\nIn file: {}", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents)
    */

    // lst 12-5 extracting parse_config fun from main
    /*
    #![allow(unused_variables)]
    use std::env;
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
   
    fn parse_config(args: &[String]) -> (&str, &str) {
        let query = &args[1];
        let filename = &args[2];
        (query, filename)
    }
     */

     // lst 12-6 refactoring parse_config to return an instance of Config struct
     /*
    #![allow(unused_variables)]
    use std::env;
    use std::fs;
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("Search for: {}\nIn file: {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
            .expect("Something went wrong reading the file");

    struct Config {
        query: String,
        filename: String,
    }

    fn parse_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
    */

    // lst 12-7 changing parse_config into Config::new
    /*
    #![allow(unused_variables)]
    use std::env;
    use std::fs;
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Search for: {}\nIn file: {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
            .expect("Something went wrong reading the file");

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Config {
            let query = args[1].clone();
            let filename = args[2].clone();
            Config { query, filename }
        }
    }
     */

    // lst 12-8 check number of args
    /*
    #![allow(unused_variables)]
    use std::env;
    use std::fs;
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Search for: {}\nIn file: {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
            .expect("Something went wrong reading the file");

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Config {
            if args.len() < 3 {
                panic!("not enought arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Config { query, filename }
        }
    }
     */

    // lst 12-9 return Result
    // lst 12-10 exit with err code if create new config fail
    /*
    #![allow(unused_variables)]
    use std::env;
    use std::fs;
    use std::process;
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1); 
            }
        );
    println!("Search for: {}\nIn file: {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
            .expect("Something went wrong reading the file");

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enought arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        }
    }
     */

    // lst 12-11 extract a run fun containing the rest of the program logic
    /*
    #![allow(unused_variables)]
    use std::env;
    use std::fs;
    use std::process;
    use std::error::Error;
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1); 
            }
        );
    println!("Search for: {}\nIn file: {}", config.query, config.filename);

    fn run(config: Config) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(config.filename)?;
            //.expect("Something went wrong reading the file");
        println!("With text:\n{}", contents);
        Ok(())
    }
    run(config);

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enought arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        }
    }
    */

    // Listing 12-12: Changing the run function to return Result
    /*
    #![allow(unused_variables)]
    use std::env;
    use std::fs;
    use std::process;
    use std::error::Error;
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1); 
            }
        );
    println!("Search for: {}\nIn file: {}", config.query, config.filename);

    fn run(config: Config) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(config.filename)?;
            //.expect("Something went wrong reading the file");
        println!("With text:\n{}", contents);
        Ok(())
    }
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };

    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enought arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        }
    }
     */

    // Listing 12-13 “Moving Config and run into src/lib.rs”
    // lst 12-14 “Using the minigrep crate in src/main.rs”
    /*
     */
    #![allow(unused_variables)]
    use std::env;
    use std::process;
    use minigrep::Config;

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            //println!("Problem parsing arguments: {}", err);
            // lst 12-24 
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1); 
            }
        );
    println!("Search for: {}\nIn file: {}", config.query, config.filename);
/*
    fn run(config: Config) -> Result<(), Box<dyn Error>>{
        let contents = fs::read_to_string(config.filename)?;
            //.expect("Something went wrong reading the file");
        println!("With text:\n{}", contents);
        Ok(())
    }
 */
    if let Err(e) = minigrep::run(config) {
        //println!("Application error: {}", e);
        // lst 12-24
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
/* 
    struct Config {
        query: String,
        filename: String,
    }

    impl Config {
        fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enought arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        }
    }
*/


}
