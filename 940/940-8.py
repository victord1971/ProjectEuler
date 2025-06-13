
MOD = 1123581313
k=5
f=[0,1]
A=[]; An=[]
B=[1]; Bn=[]
for x in range(2,k+1):
    f.append(f[x-1]+f[x-2])
#print(f)
K = f[k]+1
print(K)
f.pop(0); f.pop(0)
#print(f)

# Створюємо таблицю (K x K), заповнену нулями
#Ao = [[0 for _ in range(K)] for _ in range(K)]

# Початкові умови A[0][0] = 0
if K > 1:
    #Ao[0][1] = 1; A[1][0] = 1; A[1][1] = 2
    A.append(1); A.append(2)    #; A.append(1); 
    median = 1
    print(A,B)
# Заповнення таблиці згідно з рекурсіями    
'''
for m in range(1, K):
    for n in range(m+2):
        if n < K:
            if n == 0:  '''
result = 2
for m in range(2,K):
    for i in range(0, median):
        An.append(A[i]+A[i+1])
        #Bn.append(An[i-1]-B[i-1])
    An.append(2*An[i]+A[i])
    An.append(2*An[i+1]+A[i+1])
    Bn.append(An[median]-A[median])
    #print('                ',A,B)
    Bn.append(A[median]-B[median-2])
    for i in range((median-1),0,-1):
        #print(median,i,A[i],B)
        Bn.append(B[i-1]-B[i])
    print(An,Bn)
    A=An; An=[]; median+=1
    B=Bn; Bn=[]
    # Обчислення суми S(K)
