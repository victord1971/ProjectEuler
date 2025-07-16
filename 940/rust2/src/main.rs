
fn main() {
    let k =5;
    println!("k = {}",k);
    let k2;
    let mut m =2;
    let mut f =Vec::<u64>::new(); f.push(0); f.push(1);
    loop {
        f.push(f[m-1]+f[m-2]);
        m +=1;
        if m>k {
            break;
        }
    }
    k2 =f[k]+1;
    println!("k2= {k2}");
    //let mut c: [[i64; k+1] =[0; k+1]; k+1]; 
    //println!("{:?}", c);
}



