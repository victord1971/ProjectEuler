
use std::fs;
fn main() {
    const WORDS: [&str; 8] = ["the","be","to","of","and","in","that","have"];
    let contents = fs::read_to_string("0059_cipher.txt")
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}   {}", contents.len());
    println!("{:?}", WORDS);
}
