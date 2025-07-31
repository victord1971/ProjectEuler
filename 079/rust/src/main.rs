
//use std::env;
use std::fs;

fn main() {
    const DIGITS: [usize;8] = [0, 1, 2, 3, 6, 7, 8, 9];
    //const DIGITS_: = ['4', 1, 2, 3, 6, 7, 8, 9];
    println!("{:?}", DIGITS);
    let contents = fs::read_to_string("0079_keylog.txt")
        .expect("Should have been able to read the file");
    //println!("With text:\n{contents}   {}", contents.len());
    let mut vfile =Vec::<usize>::new();
    let mut svfile;
    let mut i = 0;
    let mut one_of_three;
    let mut password = 1000;
    let mut flag;
    let mut passlenght;
    //for string in contents {
    loop  {
        //println!( "{}", &contents[i..i+3]);
        //v.push((&contents[i..i+3]) as usize);
        //let num = contents.len();
        vfile.push(contents[i..i+3].parse::<usize>().expect("Parse error"));
        i += 4;
        if i>199  {break;}
    }
    println!("{:?}", vfile);

    loop {
        svfile = vfile[1].to_string();
        let spassword = password.to_string();
        println!("{}  {}  {}",password,spassword, svfile);
        passlenght = spassword.len();
        flag = true; i=0; one_of_three = 0;
        loop {
            if svfile[one_of_three..one_of_three+1] == spassword[i..i+1] {
                //println!("ура-ура-ура  {}  {}", svfile[one_of_three..one_of_three+1], spassword[i..i+1]);   
                println!("ура-ура-ура  {}  {}", svfile, spassword);
            }
            else {
                one_of_three += 1;
            }            
            i += 1;
            if one_of_three>2 || i>passlenght-1  {break;}
        }

        password += 1; if password>1099 {break;}
    }
}
