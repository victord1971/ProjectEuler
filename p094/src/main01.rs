

fn main() {
	let mut a=2;
	let mut c;
	let mut P;
	let mut p: usize;
	let mut s;
	let mut S: usize;
	let mut sum=0;
	loop {
		c = a-1; P = a+a+c;
		if P>1_000_000_000 {break;} 
		//if a%100_000==0 {println!("Hello, {}  {}  {}", a, a, c);}
		c = a+1; P = a+a+c;
		if P>1_000 {break;} 
		//if a%10_000==0 || a>333_330_000 {
			println!("Hello, {}  {}  {}", a, a, c-2); 
			println!("Hello, {}  {}  {}", a, a, c);
		//}
		//основний алгоритм
		p = P/2;
		if p*2 != P  {a += 1; continue;} 
		println!(" {}  {}", P, p);
		s = p*(p-a)*(p-a)*(p-c);
		S = (s as f64).sqrt() as usize;
		sum = a;
		//кінець
		a += 1;
	}
}
