
struct U3 {
    a: usize,
    b: usize,
    c: usize,
    }
fn check(_xxx: &Vec<usize>,xc: usize)-> bool  {
    for _i in 0.._xxx.len() {
        if xc == _xxx[_i] { return false;}
    }
    return true;
}

fn main() {          
    //const u3: [usize; 3];
    let mut asdf = 0;
    let mut num = 0;
    let mut max_num = 0;
    let mut a; let mut b; let mut c;
    let mut a1; let mut b1; let mut c1;
    let max = 30;
    let mut _suma =Vec::<usize>::new();
    let mut _piphagor =Vec::<U3>::new();
    let _temp = U3 {a:1,b:2,c:3};
    _piphagor.push(_temp);
    println!("{}  {}  {}  ", _piphagor[0].a, _piphagor[0].b, _piphagor[0].c);
    for m in 1..max  {
        for n in 1..m  {
            //println!("  {}  {}",m,n);
            a = m*m - n*n;
            b = 2*m*n;   
            c = m*m + n*n;   
            if a+b+c ==960 {println!("  {}  {}  {}",a,b,c);}
            if a+b+c > 1000 {break;}
            if check(&_suma,c) {_suma.push(a+b+c);}        //////
            for _i in 2..999 {
                a1 = a*_i;
                b1 = b*_i;
                c1 = c*_i;
                if a1+b1+c1 > 1000 {break;}
                //_suma.push(a1+b1+c1);
                if check(&_suma,c1) {_suma.push(a1+b1+c1);}        //////
                if a1+b1+c1 ==960 {println!("  {}  {}  {}",a1,b1,c1);}
            }
        }
    }                     
    println!("{:?}  {}", _suma, _suma.len());
    _suma.sort();
    println!("{:?}  {}", _suma, _suma.len());
    for _i in 1.._suma.len() {
        if _suma[_i] == _suma[_i-1]  {
            num += 1;
        }
        else {
            if num > max_num {max_num = num; asdf = _suma[_i-1];}
            num = 0;
        }      
        //println!("  {}  {}", num, max_num);
    }                                
    println!("===={}  {}", max_num, asdf);
}
