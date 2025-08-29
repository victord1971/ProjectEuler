

use std::fs;
//use std::fmt::Write;
    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
fn main() {
    let mut _osnovy =Vec::<usize>::new();
    let mut _stepeni =Vec::<usize>::new();
    //let mut _char = String::new();
    let mut temp;
    //let mut temp2;
    let mut poch = 0;
    let mut kinec = 1;
    let contents = fs::read_to_string("0099_base_exp.txt")
        .expect("Should have been able to read the file");
    //println!("{contents}\n   {}", contents.len());
    let blabla = contents.len();
    loop {
        if kinec+1 > blabla {
            println!("{}",&contents[poch..kinec]);
            temp = contents[poch..kinec].parse().unwrap();
            _stepeni.push(temp);
            //temp2 = (temp as u8) as char;
            //write!(_char,"{temp2}").unwrap();
            break;
        }
        if &contents[kinec..kinec+1] == "," {
            //println!("                  {}",&contents[poch..kinec]);
            temp = contents[poch..kinec].parse().unwrap();
            _osnovy.push(temp);
            poch = kinec+1;
        }
        if &contents[kinec..kinec+1] == "\n" {
            //println!("                  {}",&contents[poch..kinec]);
            temp = contents[poch..kinec].parse().unwrap();
            _stepeni.push(temp);
            poch = kinec+1;
        }
        kinec+=1;
    }
    println!("{:?}   {}", _osnovy,_osnovy.len());
    println!("{:?}   {}",_stepeni,_stepeni.len());
    //let aa = gcd(24,12);
    //println!("{}",aa);
    //let a ;
    let mut y = _osnovy[0];
    let mut tf = true;
    for x in _osnovy {
        if tf {tf = false;}
        else {y = gcd(y,x)} 
        println!("{}",y);
    }

}
