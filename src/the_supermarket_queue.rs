// 贪心，每次将顾客往等待时间最短的队列加
fn queue_time(customers: &[u32], n: u32) -> u32 {
    let mut workers = vec![0; n as usize];
    customers.iter().for_each(|x| {
        *workers.iter_mut().min().unwrap() += x;
    });
    *workers.iter().max().unwrap()
}
