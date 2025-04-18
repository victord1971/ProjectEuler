a=[2]
for num in range(3,5_00_000):
	t=0
	for x in a:
		if num%x==0:
			t=1
			break
	if t>0:
		continue
	a.append(num)
	#print(num)
print(a)
sum=0
its=0
for num in a:
	print(num)
	snum=str(num)
	t=0
	for digit in range(1,len(snum)):
		if snum[digit:digit+1] != '0':
			te1=snum[0:digit]
            #te1=snum[0:digit]
			te2=snum[digit:]
			#te2=snum[digit:]
			print('    ', te1, te2)
			if not(int(te1) in a):
				t+=1; break
			if not(int(te2) in a):
				t+=1; break
	if t==0 and num>22:
		print('            ',num)
		its+=1
print(its)		



