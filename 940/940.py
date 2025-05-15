#MOD = 1
MOD = 1123581313
k=50
f=[0,1]
for x in range(2,k+1):
    f.append(f[x-1]+f[x-2])
print(f)
K = f[k]+1
print(K)
f.pop(0); f.pop(0)
print(f)

# Створюємо таблицю (K x K), заповнену нулями
A = [0 for _ in range(K)]

# Початкові умови A[0][0] = 0
if K > 1:
    A[1] = 1
print(A, A[0], A[1], A[2])

# Заповнення таблиці згідно з рекурсіями
#for m in range(1, K):