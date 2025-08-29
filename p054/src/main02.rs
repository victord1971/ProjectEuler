
use std::fs;
//use std::fmt::Write;
fn main() {
    //let mut temp;
    let mut temp2 :&str;
    let mut poch = 0;
    let mut kinec = 1;
    struct  strr: [char;30];
    let mut _cards = strr::new();
    //let mut cards: [strr;1000];
    let contents = fs::read_to_string("0054_poker.txt")
        .expect("Should have been able to read the file");
    println!("{contents}\n   {}", contents.len());  
    let blabla = contents.len();
    let i=0;
    loop {
        if kinec+1 > blabla {
            //println!("{}",&contents[poch..kinec]);
            //temp2 = (temp as u8) as char;
            //write!(_char,"{temp2}").unwrap();
            break;
        }
        if &contents[kinec..kinec+1] == "\n" {
            //println!("                  {}",&contents[poch..kinec]);  
            //temp = contents[poch..kinec].parse().unwrap();
            temp2 = &contents[poch..kinec];
            //write!(_cards,"{temp2}").unwrap();
            //cards[i] = temp2; println!("{}", cards[i]); i += 1;
            poch = kinec+1;
        }
        kinec+=1;
    }                            
    println!("{:?}   {}", _cards,_cards.len());
    //println!("{}", cards);
}
