fn create_phone_number(numbers: &[u8]) -> String {
    let s: String = numbers.iter().map(|x| x.to_string()).collect();
    format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}
