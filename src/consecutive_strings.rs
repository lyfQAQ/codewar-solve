fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 || strarr.len() < k || k <= 0 {
        return "".to_string();
    }
    strarr
        .windows(k)
        .map(|s| s.concat())
        .rev()
        .max_by_key(String::len)
        .unwrap()
}
