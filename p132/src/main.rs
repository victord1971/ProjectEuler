// main.rs
// Знаходить перші 40 простих дільників R(10^9) і їхню суму.
// Компільувати: cargo run --release

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

// Miller-Rabin primality test deterministic for u64
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23] {
        if n == p { return true; }
        if n % p == 0 { return false; }
    }
    let d = n - 1;
    let mut s = 0u32;
    let mut d = d;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // bases for deterministic test for u64
    let bases: &[u64] = &[2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    'outer: for &a in bases.iter() {
        if a % n == 0 { continue; }
        let mut x = mod_pow(a as u128, d, n as u128) as u64;
        if x == 1 || x == n - 1 { continue; }
        for _ in 1..s {
            x = ((x as u128 * x as u128) % (n as u128)) as u64;
            if x == n - 1 { continue 'outer; }
        }
        return false;
    }
    true
}

fn main() {
    const TARGET_COUNT: usize = 40;
    const EXPONENT: u64 = 1_000_000_000u64;
    let mut found: Vec<u64> = Vec::with_capacity(TARGET_COUNT);
    // skip 2,5,3 explicitly; start from 7 and test odd numbers
    let mut p: u64 = 2;
    // We'll iterate sequentially; is_prime handles small p.
    while found.len() < TARGET_COUNT {
        if is_prime(p) {
            if p != 2 && p != 3 && p != 5 {
                let modu = (9u128).checked_mul(p as u128).expect("mod overflow");
                // compute 10^EXPONENT mod (9*p)
                let r = mod_pow(10u128, EXPONENT, modu);
                if r == 1u128 {
                    found.push(p);
                    println!("found {}: {}", found.len(), p);
                }
            }
        }
        p += 1;
        // skip even numbers >2 to speed a bit
        if p > 2 && p % 2 == 0 { p += 1; }
    }

    let sum: u128 = found.iter().map(|&x| x as u128).sum();
    println!("\nPrimes ({}): {:?}", TARGET_COUNT, found);
    println!("Sum = {}", sum);
}
