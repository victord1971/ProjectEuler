
def sieve(n: int) -> list[int]:
    is_prime = [True] * (n + 1)
    primes = []

    is_prime[0] = False
    if n >= 1:
        is_prime[1] = False

    for p in range(2, n + 1):
        if is_prime[p]:
            primes.append(p)
            for multiple in range(p * p, n + 1, p):
                is_prime[multiple] = False

    return primes

# приклад використання
n = 1_000_000_000
print(f"Прості числа до {n}:", sieve(n))
