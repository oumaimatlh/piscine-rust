pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // C = (F - 32)* (5/9)
    (f - 32.0 ) / (9.0/ 5.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    // F = (C * 9/5)+32
    (c * 9.0/5.0)+32.0


}
