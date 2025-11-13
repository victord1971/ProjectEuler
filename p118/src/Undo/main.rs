
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
    let n = 100_000;
    let primes = sieve(n);
    //println!("Прості числа до {}: {:?}", n, primes);
    
    let mut num = 123456789;
    loop
    {
		if primes.contains(&num) {
			println!("  {}", num);
		}	
		num += 1;
		if num > 987654321 {break;}
	}	
}
