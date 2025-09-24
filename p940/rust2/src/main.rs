
use std::collections::HashSet;
use std::mem;
use indicatif::{ProgressBar, ProgressStyle};

const MOD: i64 = 1_123_581_313;
const K: usize = 50;

fn compute_fib_up_to(k: usize) -> (Vec<u64>, HashSet<usize>) {
    let mut ff: Vec<u64> = vec![0u64; k - 1];
    ff[0] = 1;
    ff[1] = 2;
    for i in 2..(k - 1) {
        ff[i] = ff[i - 1].wrapping_add(ff[i - 2]);
    }
    let fib_set: HashSet<usize> = ff.iter().map(|&x| x as usize).collect();
    (ff, fib_set)
}

fn main() {
    let (ff, fib_set) = compute_fib_up_to(K);
    let k2 = ff[K - 2] as usize + 1;

    let mut a1: Vec<i64> = vec![1, 2];
    let mut b1: Vec<i64> = vec![1];

    let mut sum2: i64 = 2;
    let mut current_m: usize = 2;
    let mut median: usize = 1;

    let mut next_a: Vec<i64> = Vec::new();
    let mut next_b: Vec<i64> = Vec::new();

    // --- прогрес-бар ---
    let bar = ProgressBar::new(k2 as u64);
    bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} m | sum {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    while current_m < k2 {
        next_a.clear();
        next_b.clear();

        let is_m_in_ff = fib_set.contains(&current_m);

        for i in 0..median {
            let a_i = a1.get(i).cloned().unwrap_or(0);
            let a_ip1 = a1.get(i + 1).cloned().unwrap_or(0);
            let tmp = (a_i + a_ip1).rem_euclid(MOD);

            if fib_set.contains(&i) && is_m_in_ff {
                sum2 = (sum2 + tmp).rem_euclid(MOD);
            }
            next_a.push(tmp);
        }

        let last_idx = next_a.len().saturating_sub(1);
        let a_last = next_a.get(last_idx).cloned().unwrap_or(0);
        let a_prev = a1.get(last_idx).cloned().unwrap_or(0);
        let tmp2 = (2i64.wrapping_mul(a_last) + a_prev).rem_euclid(MOD);
        next_a.push(tmp2);
        if is_m_in_ff && fib_set.contains(&(last_idx + 1)) {
            sum2 = (sum2 + tmp2).rem_euclid(MOD);
        }

        let a_median = a1.get(median).cloned().unwrap_or(0);
        let next_a_median = next_a.get(median).cloned().unwrap_or(0);

        let diff = (next_a_median - a_median).rem_euclid(MOD);
        next_b.push(diff);

        let b0 = b1.get(0).cloned().unwrap_or(0);
        let diff2 = (a_median - b0).rem_euclid(MOD);
        next_b.push(diff2);

        if is_m_in_ff {
            sum2 = (sum2 + diff + diff2).rem_euclid(MOD);
        }

        mem::swap(&mut a1, &mut next_a);
        mem::swap(&mut b1, &mut next_b);

        median += 1;
        current_m += 1;

        //bar.inc(1);
        bar.set_message(format!("{}", sum2.rem_euclid(MOD)));
    }

    bar.finish_and_clear();
    println!("S({}) = {}", K, sum2.rem_euclid(MOD));
}
