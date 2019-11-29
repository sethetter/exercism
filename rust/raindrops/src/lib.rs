pub fn raindrops(n: u32) -> String {
    let pling: &str = if n % 3 == 0 { "Pling" } else { "" };
    let plang: &str = if n % 5 == 0 { "Plang" } else { "" };
    let plong: &str = if n % 7 == 0 { "Plong" } else { "" };

    let s: &str = &[pling, plang, plong].join("");
    if s == "" { n.to_string() } else { s.to_string() }
}
