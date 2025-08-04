
struct U3 {
    a: usize,
    b: usize,
    c: usize,
    }
fn main() {          
    //const u3: [usize; 3];
    let mut _piphagor =Vec::<U3>::new();
    let _temp = U3 {a:1,b:2,c:3};
    _piphagor.push(_temp);
    println!("{}  {}  {}  ", _piphagor[0].a, _piphagor[0].b, _piphagor[0].c);
    println!("Hello, world!");
}
