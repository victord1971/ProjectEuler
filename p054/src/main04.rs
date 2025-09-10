
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
	let mut num = 0;
    for x in _cards {
        if ranked8(&x) || ranked8(&x[15..29]) {   
			num += 1;
			println!("   {} = Four",x);
		}
        if ranked6(&x) || ranked6(&x[15..29]) {   
			num += 1;
			println!("   {} = Flush",x);
		}
		if ranked5(&x) || ranked5(&x[15..29]) {   
			num += 1;
			println!("   {} = Straight",x);
		}
		//if ( ranked5(&x) && ranked6(&x) ) || ( ranked5(&x[15..29]) && ranked6(&x[15..29]) ) {   
		//	println!("   {} = StraightFlush",x);
		//}
        if ranked4(&x) || ranked4(&x[15..29]) {   
			num += 1;
			println!("   {} = Three",x);
		}
    }  
	println!(" Загалом: {} випадків",num);
}

fn card_value(cha: &str) -> u8 {
	if *cha == *"3" {return 3}
	if *cha == *"4" {return 4}
	if *cha == *"5" {return 5}
	if *cha == *"6" {return 6}
	if *cha == *"7" {return 7}
	if *cha == *"8" {return 8}
	if *cha == *"6" {return 9}
	if *cha == *"T" {return 10}
	if *cha == *"J" {return 11}
	if *cha == *"Q" {return 12}
	if *cha == *"K" {return 13}
	if *cha == *"A" {return 14}
	2
}
fn ranked4(_hand: &str) -> bool {  //Three
	let mut num = 0;
	if _hand[0..1] == _hand[3..4] {num = 1;}
	if _hand[0..1] == _hand[6..7] {num += 1;}
	if _hand[0..1] == _hand[9..10] {num += 1;}
	if _hand[0..1] == _hand[12..13] {num += 1;}
	if num == 2 {return true}
	num = 0;
	if _hand[3..4] == _hand[6..7] {num = 1;}
	if _hand[3..4] == _hand[9..10] {num += 1;}
	if _hand[3..4] == _hand[12..13] {num += 1;}
	if num == 2 {return true}
	num = 0;
	if _hand[6..7] == _hand[9..10] {num = 1;}
	if _hand[6..7] == _hand[12..13] {num += 1;}
	if num == 2 {return true}
    false
}	
fn ranked5(_hand: &str) -> bool {  //Straight 
	let mut _ch = Vec::<u8>::new();
	_ch.push(card_value(&_hand[0..1]));
	_ch.push(card_value(&_hand[3..4]));
	_ch.push(card_value(&_hand[6..7]));
	_ch.push(card_value(&_hand[9..10]));
	_ch.push(card_value(&_hand[12..13])); _ch.sort();
    //println!(" 	{:?} ", _ch);
    if _ch[4] - _ch[0] == 4 && _ch[0] != _ch[1] && _ch[1] != _ch[2] 
							&& _ch[2] != _ch[3] && _ch[3] != _ch[4] {return true}
    false
}
fn ranked6(_hand: &str) -> bool {  //Flush
    if _hand[1..2] == _hand[4..5] &&
       _hand[1..2] == _hand[7..8] &&
       _hand[1..2] == _hand[10..11] &&
       _hand[1..2] == _hand[13..14] {return true}
    false
}
fn ranked8(_hand: &str) -> bool {  //Four
	let mut num = 0;
	if _hand[0..1] == _hand[3..4] {num = 1;}
	if _hand[0..1] == _hand[6..7] {num += 1;}
	if _hand[0..1] == _hand[9..10] {num += 1;}
	if _hand[0..1] == _hand[12..13] {num += 1;}
	if num == 3 {return true}
	num = 0;
	if _hand[3..4] == _hand[6..7] {num = 1;}
	if _hand[3..4] == _hand[9..10] {num += 1;}
	if _hand[3..4] == _hand[12..13] {num += 1;}
	if num == 3 {return true}
    false
}


