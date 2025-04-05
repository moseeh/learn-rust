use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum:: <i32>() as f64/ list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut l = list.to_vec();
    l.sort();
    let mid = l.len()/2;
    if l.len() % 2 == 0 {
        (l[mid - 1] + l[mid]) / 2
    } else {
        l[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut res = HashMap::new();
    for &value in list {
        *res.entry(value).or_insert(0) += 1;
    }
    let mut prev = 0;
    let mut repeated = 0;
    for (&key, &value) in res.iter() {
        if value > prev {
            repeated = key;
            prev = value;
        }
    }
    repeated
}