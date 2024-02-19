fn rgb(r: i32, g: i32, b: i32) -> String {
    let nums = vec![r, g, b];
    let nums: Vec<_> = nums
        .into_iter()
        .map(|x| {
            if x < 0 {
                0
            } else if x > 255 {
                255
            } else {
                x
            }
        })
        .collect();
    format!("{:02X}{:02X}{:02X}", nums[0], nums[1], nums[2])
}

#[test]
fn test_rgb() {
    println!("{}", rgb(1, 2, 3));
}
