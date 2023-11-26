/*
--------
* Modules are organized hierarchically
  - Use `super` to go up one level
  - Use `crate` to start from the top
* The `as` keyword can be used to create an alias for a module
* THe `mod` keyword is used to declare a module
  - No curly braces for external modules
* Modules can be re-exported with the use keyword
* `pub` indicateds the module maybe accessed from anyware
  - Omitted `pub` restricts access to only the containning module and sub-modules
--------
src
    -> bin -> app.rs
    -> lib 
           -> mod.rs ``` pub mod codec; pub mod transcode; ```
           -> transcode.rs
           -> codec
                    -> mod.rs ``` pub mod audio; pub mod video; ```
                    // reexport
                    // pub mod audio; pub mod video; pub use audio::mp3;
                    // use crate::codec::mp3;
                    -> audio
                             -> mod.rs ```pub mod flac; pub mod mp3;```
                             -> flac.rs
                             -> mp3.rs
                    -> video
                             -> mod.rs
                             -> h264.rs
                             -> vp9.rs

```
Cargo.toml
[[bin]]
name = "bin_name"
path = "src/bin_file.rs

[lib]
name = "lib_name"
path = "src/lib_dir/mod.rs"
```
*/
// flac.rs
use super::mp3;
pub fn sample() {
    mp3::some_fn();
    super::mp3::some_fn();
    crate::code::audio::mp3::some_fn();
    super::super::video::h264::some_fn();
}

pub fn sample() {
    use crate::transcode as tc;
    tc::some_fn();
}
