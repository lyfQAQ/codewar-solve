pub fn range_extraction(a: &[i32]) -> String {
    let (mut start, mut end) = (0usize, 0usize);
    let mut sv = Vec::new();
    while start < a.len() {
        if end < a.len() && a[end] == a[start] + (end - start) as i32 {
            end += 1;
            continue;
        }
        let mut token = String::new();
        token.push_str(a[start].to_string().as_str());
        if end - start == 2 {
            token.push(',');
            token.push_str(a[start + 1].to_string().as_str());
        }
        if end - 1 != start && end - start > 2 {
            token.push('-');
            token.push_str(a[end - 1].to_string().as_str());
        }
        sv.push(token);
        start = end;
    }

    sv.join(",")
}

#[test]
fn test_f() {
    let a = &[-3, -2, -1, 2, 10, 15, 16, 18, 19, 20];
    println!("{}", range_extraction(a));
}
