// src/main.rs
pub fn hello_from_main() { println!("hello from main"); }

fn main() {
    // binary crate
    hello_from_main();          // work, use from main binary crate in same scope 
    crate::hello_from_main();   // work, use from main binary crate as current crate
                                // The crate keyword refers to the current crate.
    //crate_both_main_lib_export::hello_from_main(); // not work, because Cargo.toml define package
                                                     // name = lib name, so hello_from_main no exist
                                                     // in lib mod
    //::hello_from_main();      // not work, same as upper line. 
                                // Paths starting with :: must reference an external crate.
    // lib crate
    //crate_both_main_lib_export::lib_mod::hello_from_lib();   // work, because pkg name = lib mod
    //::crate_both_main_lib_export::lib_mod::hello_from_lib(); // work, same upper line
    //crate::lib_mod::hello_from_lib(); // not work, crate keyword refers to the current binary crate
    //lib_mod::hello_from_lib(); // not work, lib_mod is not imported into current scope

    // import then use
    //use lib; // not work because lib mod is not external crate
    //use crate_both_main_lib_export; // same with uppers
    //crate_both_main_lib_export::lib_mod::hello_from_lib();
    // mod lib; // must list `mod lib;` in header, mean import lib.rs as `lib` mod
    //lib::lib_mod::hello_from_lib();

    // define 'libname' to src/lib.rs in Cargo.toml
    libname::lib_mod::hello_from_lib();   // work, because pkg name = lib mod
    ::libname::lib_mod::hello_from_lib(); // work, same upper line
}

//mod lib;
/*
warning: found module declaration for lib.rs
 --> src/main.rs:4:1
  |
4 | mod lib;
  | ^^^^^^^^
  |
  = note: lib.rs is the root of this crate's library target
  = help: to refer to it from other targets, use the library's name as the path
  = note: `#[warn(special_module_name)]` on by default

warning: `crate_both_main_lib_export` (bin "crate_both_main_lib_export") generated 1 warning
*/

