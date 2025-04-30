size=6
A=[size*[0] for x in range(size)]
A[0][1]=1
#print(A)   A[1][0]=1   print(A[0+1][0])
for q in range(size-1):
    A[q+1][0]=A[q][1]+A[q][0]
    print(A[q])



