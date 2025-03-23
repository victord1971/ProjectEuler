
def isPalind(m):
	mm=m
	rr=0
	while m>9:
		temp=m%10
		rr=10*rr+temp
		m=(m-temp)/10
	rr=10*rr+m
	return mm ==int(rr)
sum=0
for num in range(1,1000000):
	bi=str(bin(num)); bi=bi[2:]
	print(num,isPalind(num),bi,isPalind(int(bi)))
	if isPalind(num) and isPalind(int(bi)):
		print(88888)
		sum+=num
print(num)

