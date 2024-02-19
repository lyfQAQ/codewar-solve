fn tower_builder(n_floors: usize) -> Vec<String> {
    let mut res = vec![];
    for i in 1..=n_floors {
        let line = format!(
            "{}{}{}",
            " ".repeat(n_floors - i),
            "*".repeat(2 * i - 1),
            " ".repeat(n_floors - i)
        );
        res.push(line);
    }
    res
}



fn tower_builder_1(n_floors: usize) -> Vec<String> {
    let mut white = n_floors as i32 - 1;
    let mut star = 1usize;
    let mut res = vec![];
    for _ in 0..n_floors {
        let mut line = String::new();
        for _ in 0..white {
            line.push(' ');
        }
        for _ in 0..star {
            line.push('*');
        }
        for _ in 0..white {
            line.push(' ');
        }
        res.push(line);
        white -= 1;
        star += 2;
    }
    res
}

#[test]
fn test() {
    println!("{:?}", tower_builder(3));
}
