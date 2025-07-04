
fn main() {
    let modu: i64 =1123581313;
    let k =50;
    let k2;
    let mut m =2;
    let mut f =Vec::<u64>::new(); f.push(0); f.push(1);
    let mut a1 =Vec::<i64>::new();    a1.push(1);  a1.push(2);
    let mut b1 =Vec::<i64>::new();    b1.push(1);
    //println!("! {} {} {} {} {}",modu,k,f[0],f[1],b1[0]);
    loop {
        f.push(f[m-1]+f[m-2]);
        m +=1;
        if m>k {
            //println!("! {} {}",m,k);
            break;
        }
    }
    //println!("! {} {} ",f[2],f[3]); //,f[4],f[5],f[6],f[7]);
    k2 =f[k]+1;
    println!("! {k2}");
    f.remove(0);  f.remove(0);
    //
    m =2;
    let mut median =1;
    let mut sum =2;
    let mut i;
    let mut an =Vec::<i64>::new();
    let mut bn =Vec::<i64>::new();
    let mut tmp: i64;
    loop {       
         i =0;
         loop {
              //println!("loop! {} {}",a1[i],a1[i+1]);
              an.push( (a1[i]+a1[i+1]) %modu);
              i +=1;  if i==median {i -=1; break;}
         }
     //*                                    
         //println!("an! {}",i);
         an.push( (2*an[i]+a1[i]) %modu);
         an.push( (2*an[i+1]+a1[i+1]) %modu);
         //println!("                ! {} {}",an[median],a1[median]);
         tmp =an[median]-a1[median]; loop{if tmp<0{tmp+=modu}else{break;}}
         bn.push( tmp );
         //println!("                ! {} {}",a1[median],b1[0]);
         tmp =a1[median]-b1[0];      loop{if tmp<0{tmp+=modu}else{break;}}
         bn.push( tmp );
         for _i in 0..(median-1) {
             //println!("loop2      {} {}", b1[_i], b1[_i+1]);
             tmp = b1[_i]-b1[_i+1];  loop{if tmp<0{tmp+=modu}else{break;}}
             bn.push( tmp );
         }
         //print!("after loop2 ");
         for _i in 0..an.len() {
             //print!(" {} ",an[_i]);
         }   //println!();
         //print!("            ");
         for _i in 0..bn.len() {
             //print!(" {} ",bn[_i]);
         }   //println!();
         //a1.clear();
         a1=an.to_vec(); an.clear();
         median +=1;
         //b1.clear();
         b1=bn.to_vec(); bn.clear();
         //Обчислення суми S(K)
         if f.contains(&(m.try_into().unwrap())) {
             println!("             Обчислення {} {}",m,sum);
             for _i in 1..(m+1) {
                 if f.contains(&(_i.try_into().unwrap())) { 
                     //print!("            sum={sum}");
                     sum = (sum+a1[_i]) %modu;
                     //println!(" + {} = {}",a1[_i],sum);
                 }
             }
             //for _i in ((b1.len()-1)..0).rev() {
             let mut ii;
             ii =b1.len();
             loop {
                 ii -=1;
                 //println!(" bb {} => {}",ii,b1[ii]);
                 if f.contains(&(ii.try_into().unwrap())) {
                     //print!("            sum b={sum}");
                     sum = (sum+b1[b1.len()-1-ii]) %modu;
                     //println!(" + {} = {}",b1[ii],sum);
                 }
                 if ii==0 {break;}
             }
         }
         m +=1;  if m==k2.try_into().unwrap() {break;}
    }
    println!("! !      {}",sum);
}







