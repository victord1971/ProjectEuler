
fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let mut primes = Vec::new();

    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }

    for p in 2..=n {
        if is_prime[p] {
            primes.push(p);
            let mut multiple = p * p;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += p;
            }
        }
    }

    primes
}

fn main() {
    let n = 1000_000;
    let primes = sieve(n);
    println!("Прості числа до {}: {:?}", n, primes);
}
