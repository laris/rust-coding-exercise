#[derive(Debug)]
struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 0.0 }
    }
}

fn main() {
    let pkg = Package::default();
    println!("Default Package weight: {:?}", pkg);
    let pkg = Package::new(128.0);
    println!("Package weight: {:?}", pkg);
}
