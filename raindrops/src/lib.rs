pub fn raindrops(n: u32) -> String {
    let raindrop_factor_strings = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let rain_str: String = raindrop_factor_strings
        .iter()
        .filter(|&(num, _)| n % num == 0)
        .map(|&(_, s)| s)
        .collect();

    return if rain_str.is_empty() {
        n.to_string()
    } else {
        rain_str
    };
}
