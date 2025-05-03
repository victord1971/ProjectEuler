#MOD = 1
MOD = 1123581313
K = 6

# Створюємо таблицю (K x K), заповнену нулями
A = [[0 for _ in range(K)] for _ in range(K)]

# Початкові умови
if K > 1:
    A[0][1] = 1
    A[1][0] = 1
for n in range(2, K):
    A[0][n] = (A[0][n-1] + A[0][n-2]) #% MOD
    A[n][0] = A[0][n]

# Заповнення таблиці згідно з рекурсіями
for m in range(1, K):
    for n in range(1, K):
        if n == 0:
            A[m][n] = (A[m-1][n] + A[m-1][n+1]) #% MOD
        else:
            A[m][n] = (2 * A[m][n-1] + A[m-1][n-1]) #% MOD

# Обчислення суми S(K)
result = 0
for m in range(1, K):
    for n in range(1, K):
        result = (result + A[m][n]) 
print(A)
print(result % MOD)

