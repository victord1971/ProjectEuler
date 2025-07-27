
//use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("0079_keylog.txt")
        .expect("Should have been able to read the file");
    //println!("With text:\n{contents}   {}", contents.len());
    let mut v =Vec::<usize>::new();
    let mut i = 0;
    //for string in contents {
    loop  {
        //println!( "{}", &contents[i..i+3]);
        //v.push((&contents[i..i+3]) as usize);
        //let num = contents.len();
        v.push(contents[i..i+3].parse::<usize>().expect("Parse error"));
        i += 4;
        if i>199  {break;}
    }
    println!("{:?}", v);
}
