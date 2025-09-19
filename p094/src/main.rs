

fn main() {
	let mut boo: bool;
	let mut a=2;
	let mut c;
	let mut P;
	let mut p: usize;	//напівпериметр
	let mut S: u128;	//площа
	let mut sum=0;
	loop {
		c = a-1; P = a+a+c;
		if P>1_000_000_000 {break;} 
		//if a%100_000==0 {println!("Hello, {}  {}  {}", a, a, c);}
		//основний алгоритм
		p = P/2;
		//println!(" {}  {}", P, p);
		if p*2 != P  {a += 1; continue;} 
		(S,boo) = resolve(a, c, p);
		if boo {println!("       S= {}", S);sum += P;}
		
		//кінець
		c = a+1; P = a+a+c;
		if P>1_000_000_000 {break;} 
		//основний алгоритм
		p = P/2;
		//println!(" {}  {}", P, p);
		if p*2 != P  {a += 1; continue;} 
		(S,boo) = resolve(a, c, p);
		if boo {println!("       S= {}", S);sum += P;}
		
		//кінець
		a += 1;
	}
	println!(" sum= {}", sum);
}
fn resolve(a: usize, c: usize, p: usize)-> (u128, bool) {
	let mut boo: bool= false;
	let mut s: u128;
	let mut S: u128;
	let aa = a as u128;
	let cc = c as u128;
	let pp = p as u128;
	s = pp*(pp-aa)*(pp-aa)*(pp-cc);
	//println!("s= {}", s);
	S = (s as f64).sqrt() as u128;
	if S*S == s {boo= true;}
	return (S, boo);
}
