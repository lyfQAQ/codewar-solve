fn to_camel_case(text: &str) -> String {
    text.split(&['-', '_'])
        .enumerate()
        .map(|(i, w)| match i {
            0 => w.to_string(),
            _ => w[..1].to_uppercase() + &w[1..],
        })
        .collect()
}

fn to_camel_case_(text: &str) -> String {
    let tokens: Vec<_> =text.split(['_', '-']).collect();
    let mut res = String::new();
    res.push_str(tokens[0]);
    for i in 1..tokens.len() {
        let cur = tokens[i];
        res.push_str(cur[0..1].to_uppercase().as_str());
        res.push_str(&cur[1..]);
    }
    res
}