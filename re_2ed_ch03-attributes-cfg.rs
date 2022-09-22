fn main() {
  on_windows();
}

//#[cfg(target_os = "windows")]
#[cfg(target_os = "macos")]
fn on_windows() {
  println!("This machine has Windows as its OS.");
}

