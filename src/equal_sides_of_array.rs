fn find_even_index(arr: &[i32]) -> Option<usize> {
    let mut result = None;
    let mut left_sum = 0;
    let sum = arr.iter().sum::<i32>();
    for (i, &x) in arr.iter().enumerate() {
        if left_sum == sum - left_sum {
            result = Some(i - 1);
            break;
        }
        left_sum += x;
    }
    result
}
