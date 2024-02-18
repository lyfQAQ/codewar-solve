fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut res = vec![];
    for item in sequence {
        if res.is_empty() || item != res.last().unwrap(){
            res.push(item);
        }
    }
    res
}

fn unique_in_order_1<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut res: Vec<_> = sequence.into_iter().collect();
    res.dedup();
    res 
}


