
use std::cmp::Ordering;

fn main() {
    let mut items = vec![4.5, 11.5, -7.3, 14.0, 18.7, 11.5, 1.3, -2.1, 33.7];
    println!("{:?}", items);
    items.sort_by(cmp_f64);
    println!("{:?}", items);
}

fn cmp_f64(a: &f64, b: &f64) -> Ordering {
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}