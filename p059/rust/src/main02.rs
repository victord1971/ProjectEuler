
use std::fmt::Write;
use std::fs;
fn main() {
    let mut temp;
    let mut temp2;
    let mut _content =Vec::<u8>::new();
    let mut _char = String::new();
    let mut _passw = String::new();
    let mut poch = 0;
    let mut kinec = 1;
    const WORDS: [&str; 8] = ["the","be","to","of","and","in","that","have"];
    let contents = fs::read_to_string("0059_cipher.txt")
        .expect("Should have been able to read the file");
    //println!("With text:\n{contents}   {}", contents.len());
    println!("   {}", contents.len());
    println!("{:?}", WORDS);
    println!("{:?}", WORDS);
    println!("{}",65^42);
    println!("{}",107^42);
    println!("{}",65^107);
    //println!("{}",&contents[0..30]);
    loop {
        if kinec+1 > contents.len() {   
            //println!("{}",&contents[poch..kinec]);
            temp = contents[poch..kinec].parse().unwrap();
            _content.push(temp);           
            temp2 = (temp as u8) as char;
            write!(_char,"{temp2}").unwrap();
            break;
        }
        if &contents[kinec..kinec+1] != "," {}
        else {
            //println!("{}",&contents[poch..kinec]);
            temp = contents[poch..kinec].parse().unwrap();
            _content.push(temp);
            temp2 = (temp as u8) as char;
            write!(_char,"{temp2}").unwrap();
            poch = kinec+1;
        }
        kinec+=1;
    }         
    println!("{:?}", _content);   
    println!("{}",_content.len());    
    println!("{:?}", _char);           
    println!("{}",_char.len());

    for x in 97..123 {
        for y in 97..123 {
            for z in 97..123 {
                _passw.push(x as u8 as char);
                _passw.push(y as u8 as char);
                _passw.push(z as u8 as char);
                //println!("{:?}", _passw);

                _passw.clear();
            }
        }
    }
}
