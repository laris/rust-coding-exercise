trait Clicky { fn click(&self); }
#[derive(Debug)]
struct Keyboard;
impl Clicky for Keyboard { fn click(&self) { println!("click clack"); } }
fn borrow_clicky(obj: &dyn Clicky) { obj.click(); }
fn move_clicky(obj: Box<dyn Clicky>) { obj.click(); }
#[derive(Debug)]
struct Mouse;
impl Clicky for Mouse { fn click(&self) { println!("mouse click"); } }
fn make_clicks(clickers: Vec<Box<dyn Clicky>>) {
    for clicker in clickers { clicker.click(); } }
fn main() {
    let kbd = Keyboard;
    let kbd_obj: &dyn Clicky = &kbd;
    let kbd: &dyn Clicky = &Keyboard;
    let kbd: Box<dyn Clicky> = Box::new(Keyboard);
    let kbd = Keyboard;
    borrow_clicky(&kbd);
    let kbd = Box::new(Keyboard);
    move_clicky(kbd);
    let keeb: Box<dyn Clicky> = Box::new(Keyboard);
    let mice: Box<dyn Clicky> = Box::new(Mouse);
    let clickers = vec![keeb, mice];
    //println!("{clickers:?}");
    let keeb = Box::new(Keyboard);
    let mice = Box::new(Mouse);
    let clickers: Vec<Box<dyn Clicky>> = vec![keeb, mice];
    //
    println!("fn make_clicks()");
    make_clicks(clickers);
}
