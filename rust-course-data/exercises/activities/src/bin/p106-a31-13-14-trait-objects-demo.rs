trait Sale {
    fn amount(&self) -> f64;
}

struct FullScale(f64);
impl Sale for FullScale { fn amount(&self) -> f64 { self.0 } }
struct OneDollarOffCoupon(f64);
impl Sale for OneDollarOffCoupon { fn amount(&self) -> f64 { self.0 - 1.0 } }
struct TenPercentOffCoupon(f64);
impl Sale for TenPercentOffCoupon { fn amount(&self) -> f64 { self.0 * 0.9 } }

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 { sales.iter().map(|sale| sale.amount()).sum() }
fn main() {
    let price = 20.0;
    let regular = Box::new(FullScale(price));
    let coupon = Box::new(OneDollarOffCoupon(price));
    let promo = Box::new(TenPercentOffCoupon(price));
    let sales: Vec<Box<dyn Sale>> = vec![regular, coupon, promo];

    println!("revenue: {:?}", calculate_revenue(&sales));
}
