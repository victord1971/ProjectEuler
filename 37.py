a=[2]
sum=0
its=0
for num in range(3,5_000_000):
	
	t=0
	for x in a:
		if num%x==0:
			t=1
			break
	if t>0:
		continue
	a.append(num)
	
	snum=str(num)
	t=0
	for digit in range(1,len(snum)):
		if snum[digit:digit+1] != '0':
			te1=snum[0:digit]
            #te1=snum[0:digit]
			te2=snum[digit:]
			if not(int(te1) in a):
				t+=1; break
			if not(int(te2) in a):
				t+=1; break
	if t==0 and num>22:
		sum+=num
		print('            ',num)
		its+=1
		print(its)	
		if its==11:
			break	
print(sum)

#for num in a:
#	print(num)



