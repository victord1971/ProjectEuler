
use std::fmt::Write;
fn main() {
	let mut pr: usize;
	let mut num =1;
	let mut temp: &str;
    let mut _line = String::new();  
	//temp = &num.to_string();
	loop {
		let binding = num.to_string();
		temp = &binding;
		write!(_line,"{temp}").unwrap();  
		num += 1;
		if _line.len() > 1_000_000 {break;}
	}
	pr = _line[0..1].parse::<usize>().expect("REASON"); 
	pr *= _line[9..10].parse::<usize>().expect("REASON");
	pr *= _line[99..100].parse::<usize>().expect("REASON");
	pr *= _line[999..1000].parse::<usize>().expect("REASON");
	pr *= _line[9999..10000].parse::<usize>().expect("REASON");
	pr *= _line[99999..100000].parse::<usize>().expect("REASON");
	pr *= _line[999999..1000000].parse::<usize>().expect("REASON");
	//println!("Hello, world!  {:?}", _line);
	println!("Hello, world!  {}", pr);
}
