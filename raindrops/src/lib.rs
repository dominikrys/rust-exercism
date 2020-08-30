pub fn raindrops(n: u32) -> String {
    let mut rain_str = String::new();

    if n % 3 == 0 {
        rain_str.push_str("Pling")
    }

    if n % 5 == 0 {
        rain_str.push_str("Plang")
    }

    if n % 7 == 0 {
        rain_str.push_str("Plong")
    }

    return if rain_str.is_empty() {
        n.to_string()
    } else {
        rain_str
    };
}
