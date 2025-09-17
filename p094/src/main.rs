fn main() {
	let mut a=2;
	let mut c;
	let mut p;
	loop {
		c = a-1; p = a+a+c;
		if p>1_000_000_000 {break;} 
		//if a%100_000==0 {println!("Hello, {}  {}  {}", a, a, c);}
		c = a+1; p = a+a+c;
		if p>1_000_000_000 {break;} 
		if a%10_000==0 || a>333_330_000 {
			println!("Hello, {}  {}  {}", a, a, c-2); 
			println!("Hello, {}  {}  {}", a, a, c);
		}
		a += 1;
	}
}
