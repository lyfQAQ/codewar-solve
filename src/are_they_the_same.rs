fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>();
    let mut a2 = b;
    a1.sort();
    a2.sort();
    a1 == a2
}

#[test]
fn tests_comp() {
    // let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    // let a2 = vec![
    //     11 * 11,
    //     121 * 121,
    //     144 * 144,
    //     19 * 19,
    //     161 * 161,
    //     19 * 19,
    //     144 * 144,
    //     19 * 19,
    // ];
    // assert_eq!(comp(a1, a2), true);
    let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
    let a2 = vec![
        11 * 21,
        121 * 121,
        144 * 144,
        19 * 19,
        161 * 161,
        19 * 19,
        144 * 144,
        19 * 19,
    ];
    assert_eq!(comp(a1, a2), false);
}
