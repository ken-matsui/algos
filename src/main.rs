use std::fmt::Display;

use num::{Bounded, Integer, NumCast};

fn radix_sort<I>(arr: &mut Vec<I>)
where
    I: Copy + Integer + Bounded + NumCast + Display,
{
    let mut max: I = Bounded::min_value();
    for v in &mut *arr {
        if max < *v {
            max = *v;
        }
    }

    let num_digits = max.to_string().len() as u32;
    let ten = I::from(10).unwrap();
    for i in 0..=num_digits {
        let mut buckets: Vec<Vec<I>> = vec![vec![]; 10];

        for num in &mut *arr {
            let power = I::from(10_u64.pow(i)).unwrap();
            let floor = num.div_floor(&power);
            let (_, digit) = floor.div_rem(&ten);
            let index = I::to_usize(&digit).unwrap();
            buckets[index].push(*num);
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
