
use indicatif::{ProgressBar, ProgressStyle};

fn mod_mul(a: u128, b: u128, m: u128) -> u128 {
    (a * b) % m
}

fn mod_pow(mut base: u128, mut exp: u64, modu: u128) -> u128 {
    let mut res: u128 = 1 % modu;
    base %= modu;
    while exp > 0 {
        if (exp & 1) == 1 {
            res = mod_mul(res, base, modu);
        }
        base = mod_mul(base, base, modu);
        exp >>= 1;
    }
    res
}

// Miller–Rabin primality test (детермінований для u64)
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    let mut d = n - 1;
    let mut s = 0u32;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let bases: &[u64] = &[2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    'outer: for &a in bases {
        if a % n == 0 {
            continue;
        }
        let mut x = mod_pow(a as u128, d, n as u128) as u64;
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = ((x as u128 * x as u128) % (n as u128)) as u64;
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn main() {
    const TARGET_COUNT: usize = 40;
    const EXPONENT: u64 = 1_000_000_000u64;

    let mut found: Vec<u64> = Vec::with_capacity(TARGET_COUNT);

    let bar = ProgressBar::new(100_000); // умовна "довжина" для анімації
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7} candidates | found {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    let mut p: u64 = 7;
    while found.len() < TARGET_COUNT {
        if is_prime(p) {
            let modu = 9u128 * (p as u128);
            let r = mod_pow(10u128, EXPONENT, modu);
            if r == 1 {
                found.push(p);
                bar.set_message(format!("{}", found.len()));
                println!("✅ {}-й простий дільник: {}", found.len(), p);
            }
        }

        // оновлюємо прогрес (штучно, щоб красиво бігало)
        bar.inc(1);

        p += if p == 2 { 1 } else { 2 }; // пропускаємо парні > 2
    }
    bar.finish_and_clear();

    let sum: u128 = found.iter().map(|&x| x as u128).sum();
    println!("\nПрості дільники ({}): {:?}", TARGET_COUNT, found);
    println!("Сума = {}", sum);
}

