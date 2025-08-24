def generate_sequences(k, max_val=100):
    def dfs(seq, total):
        if len(seq) == k:
            return [total]
        res = []
        start = seq[-1] + 1 if seq else 1
        end = min(2 * seq[-1], max_val) if seq else max_val
        for i in range(start, end + 1):
            res += dfs(seq + [i], total + i)
        return res
    return dfs([], 0)

MOD = 1123581313
def S(k):
    sequences_sums = generate_sequences(k)
    return sum(sequences_sums) % MOD

print(S(5))  # Це має дати 10396
