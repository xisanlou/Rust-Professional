pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let mut p: f64 = 1.0;
    for i in 2..=n {
        p *= (365 - i + 1) as f64 / 365.0; 
    }

    1.0 - p
}
