use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut v = vec![4, 2, 1, 1, 3, 5, 5];
    println!("v is {:?}", v);
    println!("mean of v is {}", mean(&v));
    println!("median of v is {}", median(&mut v));
    println!("mode(s) of v is {:?}", mode(&v));
}

fn mean(v: &Vec<i32>) -> f64 {
    let sum = sum(v) as f64;
    sum / (v.len() as f64)
}

fn sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0i32;
    for i in v {
        sum += i;
    }
    sum
}

fn median(v: &mut Vec<i32>) -> f64 {
    v.sort();
    let is_odd = (v.len() % 2) != 0;
    if is_odd {
        v[v.len() / 2] as f64
    } else {
        ((v[v.len() / 2] + v[(v.len() / 2) - 1]) as f64) / 2.0f64
    }
}

fn mode(v: &Vec<i32>) -> Vec<i32> {
    let mut counts = HashMap::new();
    for i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    let mut highest_count = 0;
    let mut modes = Vec::<i32>::new();
    for (value, count) in counts {
        if count > highest_count {
            highest_count = count;
            modes.clear();
            modes.push(*value);
        } else if count == highest_count {
            modes.push(*value);
        }
    }
    modes
}
