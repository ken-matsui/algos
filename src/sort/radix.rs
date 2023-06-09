use crate::types::Int;

fn count_digits(num: &Int) -> u32 {
    if *num == 0 {
        1
    } else {
        num.abs().ilog10() + 1
    }
}

pub fn radix_sort(arr: &mut Vec<Int>) {
    let max = arr.iter().max().unwrap();
    let num_digits = count_digits(max);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::is_sorted;

    #[test]
    fn test_radix_sort() {
        let mut arr = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut arr);
        assert!(is_sorted(arr));
    }
}
