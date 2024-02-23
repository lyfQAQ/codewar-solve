fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut ans = 0;
    let mut intervals = intervals.to_owned();
    intervals.sort();
    let mut a = intervals[0].0;
    let mut b = intervals[0].1;
    ans = b - a;
    for (i, (x, y)) in intervals.iter().enumerate().skip(1) {
        if *x > b {
            ans += y - x;
            a = *x;
            b = *y;
        } else if *x >= a && *y <= b {
            continue;
        } else {
            ans += *y - b;
            b = *y;
        }
    }
    ans
}

#[test]
fn test_f() {
    let a = &[(1, 20), (2, 19), (5, 15), (8, 12)];
    println!("{}", sum_intervals(a));
}
