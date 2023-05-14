#![feature(int_roundings)]

mod types;

use types::Int;

fn radix_sort(arr: &mut Vec<Int>) {
    let max = arr.iter().max().unwrap();
    let num_digits = max.to_string().len() as u32;

    for i in 0..=num_digits {
        let mut buckets: Vec<Vec<Int>> = vec![vec![]; 10];

        for num in &mut *arr {
            let digit = num.div_floor(10_u64.pow(i) as i32) % 10;
            buckets[digit as usize].push(*num);
        }

        *arr = vec![];
        for bucket in buckets {
            arr.extend(bucket);
        }
    }
}

fn main() {
    let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
    radix_sort(&mut arr);
    println!("{arr:?}");
}
