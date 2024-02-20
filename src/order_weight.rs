fn order_weight(s: &str) -> String {
    let get_wight = |s: &str| s.chars().map(|ch| ch.to_digit(10).unwrap()).sum::<u32>();
    let mut ss: Vec<&str> = s.split(" ").collect();
    ss.sort_by_key(|&s| (get_wight(s), s));
    ss.join(" ")
}
