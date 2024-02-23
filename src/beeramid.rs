fn beeramid(bonus: i32, price: f32) -> usize {
    let n = (bonus as f32 / price) as i32;
    let (mut sum, mut c) = (0, 1);
    while sum + c * c <= n {
        sum += c * c;
        c += 1;
    }
    (c - 1) as usize
}
