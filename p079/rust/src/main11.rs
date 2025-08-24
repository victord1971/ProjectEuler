
use std::fs::File;
use std::io::Read;

fn main() {
    let mut content = String::new();

    let mut file = File::open("0079_keylog.txt")?;
    // считываем текст в переменную content
    file.read_to_end(&mut content)?;

    println!("{}", content);
    Ok(())
}
