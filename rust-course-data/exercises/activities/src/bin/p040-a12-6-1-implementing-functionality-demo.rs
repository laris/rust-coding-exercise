struct Temperature {
    degrees_f: f64,
}

fn show_temp(temp: &Temperature) {
    // {:>6.2}, > align, 6 total width, .2 two decimal place
    println!("{:>6.2}\tdegrees F", temp.degrees_f);
}

impl Temperature {
    fn show_temp_impl(temp: &Temperature) {
        println!("{:>6.2}\tdegrees F", temp.degrees_f);
    }
    fn show_temp_impl_self(&self) {
        println!("{:>6.2}\tdegrees F", self.degrees_f);
    }
    fn freezing() -> Self {
        Self { degrees_f: 32.00 }
    }
    fn boiling() -> Self {
        Self { degrees_f: 212.00 }
    }
}


fn main() {
    let hot = Temperature { degrees_f: 99.99, };
    show_temp(&hot);
    //hot.show_temp_impl();
    Temperature::show_temp_impl(&hot);
    hot.show_temp_impl_self();

    let cold = Temperature::freezing();
    cold.show_temp_impl_self();

    Temperature::show_temp_impl_self(&Temperature::boiling());
}
