

fn main() {
    let modu: i64 =1123581313;
    //let k =50;
    const K: usize =50;
    let k2;
    //let mut m =2;  
    let mut mm =2;  
    let mut m2;
    //let mut f =Vec::<u64>::new(); f.push(0); f.push(1);
    let mut ff: [u64; K-1] =[0; K-1]; 
    ff[0]=1; ff[1]=2; 
    let mut a1 =Vec::<i64>::new();    a1.push(1);  a1.push(2);
    let mut b1 =Vec::<i64>::new();    b1.push(1);
/*    
    loop { f.push(f[m-1]+f[m-2]);
        m +=1;
        if m>K {
            break; } }
*/            
    if mm<K-1 { loop {
        ff[mm] =ff[mm-1]+ff[mm-2];
        mm +=1;
        if mm>K-2 { break; } } }
    k2 =ff[K-2]+1;
    println!("k2= {k2}");
    //f.remove(0);  f.remove(0);
    //println!("{:?}", f);
    println!("{:?}", ff);
    // Основна частина
    mm =2;
    let mut median =1;
    //let mut sum =2;
    let mut sum2 =2;
    let mut i;
    let mut an =Vec::<i64>::new();
    let mut bn =Vec::<i64>::new();
    let mut tmp: i64;
    let mut tmp2: i64;
    let mut tmp3: i64;
    let mut flm: bool;
    let mut fli: bool;
    loop {       
         i =0; flm =false;
         //if f.contains(&(m as u64))   {
         if ff.contains(&(mm as u64))   {
              flm =true; 
         }
         loop {
              tmp =(a1[i]+a1[i+1]) %modu;
              //println!("     i= {}",i);
              if ff.contains(&(i as u64))  {
                fli =true;
                if flm {
                  sum2 +=tmp;  
                  //println!("1   +={}",tmp);
                } 
              }   
              else { fli =false; }
              an.push(tmp);
              i +=1;  
              if i==median {i -=1; break;}
         }
         tmp2 =(2*an[i]+a1[i]) %modu;
         an.push(tmp2);
         if flm && ff.contains(&((i+1) as u64))  {
              sum2 +=tmp2; 
              //println!("2   +={} ", tmp2);
         }         
         tmp3 =(2*an[i+1]+a1[i+1]) %modu;
         an.push(tmp3);
         if flm /*&& fli*/  {
            //3333333333333333333
            sum2 +=tmp3; 
            //println!("3   +={} ", tmp3);
         }         
         
         tmp2 =an[median]-a1[median]; loop{if tmp<0{tmp+=modu}else{break;}}
         bn.push( tmp2 );
         tmp3 =a1[median]-b1[0];      loop{if tmp<0{tmp+=modu}else{break;}}
         bn.push( tmp3 );
         if flm {
            m2 =mm-1;
            if ff.contains(&(m2 as u64)) {
                sum2 +=tmp2;
                //444444444444444444444444444
                //println!("4   += {}  ", tmp2); 
            }   
            if fli {
                //println!("5   += {} ", tmp3);
                sum2 +=tmp3;
            }
        }
         for _i in 0..(median-1) {
             tmp = b1[_i]-b1[_i+1];  loop{if tmp<0{tmp+=modu}else{break;}}
             bn.push( tmp );
             //666666666666666666666666666
             m2 =mm-3-_i;
             if flm && ff.contains(&(m2 as u64)) {
                //println!("6   += {} ", tmp);
                sum2 =(sum2+tmp) %modu;
                println!("             Обчислення {} {}",mm,sum2);
             }   
         }
/*
         for _i in 0..an.len() {
             print!(" {} ",an[_i]);
             println!(); }
         for _i in 0..bn.len() {
             print!(" {} ",bn[_i]);
             println!(); }
*/
         a1=an.to_vec(); an.clear();
         median +=1;
         b1=bn.to_vec(); bn.clear();
         //Обчислення суми S(K)     
/*
         if f.contains(&(m.try_into().unwrap())) {
             println!("             Обчислення {} {}",m,sum);
             for _i in 1..(m+1) {
                 if f.contains(&(_i.try_into().unwrap())) { 
                     sum = (sum+a1[_i]) %modu;
                 }
             }
             let mut ii;
             ii =b1.len();
             loop {
                 ii -=1;
                 if f.contains(&(ii.try_into().unwrap())) {
                     sum = (sum+b1[b1.len()-1-ii]) %modu;
                 }
                 if ii==0 {break;}
             }
         }  
*/
         mm +=1;  if mm==k2.try_into().unwrap() {break;}
    }
    println!("! !      {}",sum2);
}







