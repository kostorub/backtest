pub fn comission(price: f64, qty: f64, commission: f64) -> f64 {
    // commission is in percents like 1% or 0.5%
    price * qty * commission / 100.0
}
