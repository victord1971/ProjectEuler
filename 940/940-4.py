
MOD = 1123581313
K = 5

# dp[n][k] — кількість послідовностей довжини n, що закінчуються на k
dp = [[0] * (K + 1) for _ in range(K + 1)]

# Послідовності довжини 1: будь-яке число від 1 до K
for k in range(1, K + 1):
    dp[1][k] = 1

# Побудова dp для довжини від 2 до K
for length in range(2, K + 1):
    for last in range(1, K + 1):
        # Шукаємо попередні числа prev, які ≤ last і ділять last
        for prev in range(1, last + 1):
            if last % prev == 0:
                dp[length][last] = (dp[length][last] + dp[length - 1][prev]) % MOD

# A(m, n) = сума dp[n][x] для x ∈ [1..m]
def A(m, n):
    return sum(dp[n][x] for x in range(1, m + 1)) % MOD

# S(K) = сума A(m, n) для m,n = 1..K
S = 0
for m in range(1, K + 1):
    for n in range(1, K + 1):
        S = (S + A(m, n)) % MOD

print(f"S({K}) =", S)
