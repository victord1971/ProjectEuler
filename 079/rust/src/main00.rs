
use std::fs::File;
//use std::io::Read;
fn main() {
    println!("Hello, world!");
    // Відкриття файлу на читання
    //let mut file = File::open("0079_keylog.txt");
    let result = File::open("0079_keylog.txt");
    match result {
        Ok(_) =>println!("File is opened"),
        Err(_) => println!("Error occured")
    }
    
    let mut content = String::new();
    result.read_to_end(&mut content)?;

    println!("{}", content);

}
