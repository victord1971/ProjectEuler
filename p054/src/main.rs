
use std::fs;
//use std::fmt::Write;
fn main() {
    //let mut temp;
    let mut temp2 :&str;
    let mut poch = 0;
    let mut kinec = 1;
    let mut _cards = Vec::<String>::new();
    let contents = fs::read_to_string("0054_poker.txt")
        .expect("Should have been able to read the file");
    //println!("{contents}\n   {}", contents.len());  
    let blabla = contents.len();
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
            //temp2 = &contents[poch..kinec].to_string();
            //write!(_cards,"{temp2}").unwrap();
            _cards.push(temp2.to_string());
            poch = kinec+1;
        }
        kinec+=1;
    }
    //println!("{:?}   {}", _cards,_cards.len());
    for x in _cards {
        if ranked6(&x) || ranked6(&x[15..29]) {   println!("   {} = Flush",x);}
    }
}
fn ranked5(_hand: &str) -> bool {  //Straight Flush
    false
}
fn ranked6(_hand: &str) -> bool {  //Flush
    if _hand[1..2] == _hand[4..5] &&
       _hand[1..2] == _hand[7..8] &&
       _hand[1..2] == _hand[10..11] &&
       _hand[1..2] == _hand[13..14]{return true}
    false
}
