// src/main.rs
fn main() {
    // 使用绝对路径 crate:: 来引用库中的函数
    //crate::hello_from_lib();
    crate_both_main_lib_export::lib_mod::hello_from_lib();
    //hello_from_main();
}
