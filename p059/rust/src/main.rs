
use std::fmt::Write;
use std::fs;
fn main() {
    let mut temp;
    let mut temp2;
    let mut _content =Vec::<u8>::new();
    let mut _char = String::new();
    //let mut _passw = String::new();
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
            temp2 = temp as u8 as char;
            write!(_char,"{temp2}").unwrap();
            poch = kinec+1;
        }
        kinec+=1;
    }         
    println!("{:?}", _content);
    let len = _content.len();
    println!("{}", len);
    println!("{:?}", _char);           
    println!("{}",_char.len());
    let mut flag = false;
    for x in 97..123 {
        for y in 97..123 {
            for z in 97..123 {
                let passw = format!("{}{}{}", x as u8 as char,
                                              y as u8 as char,
                                              z as u8 as char);
                println!("{}", passw);     
                let mut _contend =Vec::<u8>::new();
                let mut num = 0;
                let mut i = 0;
                let mut _content2 =String::new();
                loop {
                    temp = _content[i] ^ (x as u8); if temp == 32 {num+=1;}
                    _content2.push(temp as char);
                    _contend.push(temp);
                    i += 1; if i == len { break; }

                    temp = _content[i] ^ (y as u8); if temp == 32 {num+=1;}
                    _content2.push(temp as char);     
                    _contend.push(temp);
                    i += 1; if i == len { break; }

                    temp = _content[i] ^ (z as u8); if temp == 32 {num+=1;}
                    _content2.push(temp as char);     
                    _contend.push(temp);
                    i += 1; if i == len { break; }
                }                   
                println!("{:?}", _content2);  
                println!("     {}",_contend.len());
                if num > 200 {    
                    let mut numm :usize = 0;
                    for i in 0.._contend.len() {
                        numm += _contend[i] as usize;
                    }   
                    println!("{}      {}",_contend.len(),numm);
                    flag = true;
                    break;
                }
            }
            if flag {break;}
        }     
        if flag {break;}
    }
}
