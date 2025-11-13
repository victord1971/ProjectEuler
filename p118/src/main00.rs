use itertools::Itertools;
use std::collections::HashSet;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn digits_to_num(d: &[u8]) -> u64 {
    d.iter().fold(0, |acc, &x| acc * 10 + x as u64)
}

/// Генерує всі розбиття послідовності цифр на групи
fn partitions(digits: &[u8]) -> Vec<Vec<Vec<u8>>> {
    if digits.len() == 1 {
        return vec![vec![digits.to_vec()]];
    }

    let mut result = Vec::new();
    for i in 1..digits.len() {
        let left = &digits[..i];
        let right = &digits[i..];
        for mut rest in partitions(right) {
            let mut combined = vec![left.to_vec()];
            combined.append(&mut rest);
            result.push(combined);
        }
    }
    result.push(vec![digits.to_vec()]);
    result
}

fn main() {
    let digits: Vec<u8> = (1..=9).collect();
    let mut sets: HashSet<Vec<u64>> = HashSet::new();

    for perm in digits.iter().permutations(9).unique() {
        let perm_digits: Vec<u8> = perm.into_iter().copied().collect();

        for partition in partitions(&perm_digits) {
            let nums: Vec<u64> = partition.iter().map(|p| digits_to_num(p)).collect();

            if nums.iter().all(|&n| is_prime(n)) {
                // Сортуємо, щоб [2,3,47] == [3,2,47]
                let mut sorted_nums = nums.clone();
                sorted_nums.sort_unstable();
                sets.insert(sorted_nums);
            }
        }
    }

    println!("Кількість панцифрових простих множин: {}", sets.len());
}
