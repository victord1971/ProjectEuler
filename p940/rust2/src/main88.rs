
fn main() {
    const K: usize =50;
    println!("k = {}",K);
    let k2;
    let mut m =2;
    let mut f =Vec::<u64>::new(); f.push(0); f.push(1);
    loop {
        f.push(f[m-1]+f[m-2]);
        m +=1;
        if m>K {
            break;
        }
    }
    println!("{:?}", f);
    k2 =f[K]+1;
    println!("k2= {k2}");
    let mut c: [[i64; K+1]; K+1] =[[0; K+1]; K+1]; 
    c[0][1] =1;
    println!("{:?}", c);

    //Розрахунок усих і зберігання тільки потрібних елементів у МАСИВІ 51х51
    //let mut diagonal: [i64; K+1] =[0; K+1]; 
    //const K2: usize =k2;
    //let mut diagonal: [i64; 9026] =[0; 9026]; 
    let mut diagonal: Box<[i64; 12586269026]> =Box::new([0; 12586269026]); //--release
    diagonal[0] =10;
    //println!("diagonal={:?}", diagonal);


}



