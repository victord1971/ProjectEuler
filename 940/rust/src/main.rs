
fn main() {
    let modu: u128 =1123581313;
    let k =3;
    let k2: u128;
    let mut m =2;
    let mut f =Vec::<u128>::new(); f.push(0); f.push(1);
    let mut a1 =Vec::<u128>::new();    a1.push(1);  a1.push(2);
    let mut b1 =Vec::<u128>::new();    b1.push(1);
    println!("! {} {} {} {} {}",modu,k,f[0],f[1],b1[0]);
    loop {
        f.push(f[m-1]+f[m-2]);
        m +=1;
        if m>k {
           println!("! {} {}",m,k);
            break;
        }
    }
    println!("! {} {} ",f[2],f[3]); //,f[4],f[5],f[6],f[7]);
    k2 =f[k]+1;
    f.remove(0);  f.remove(0);
    //
    m =2;
    let mut median =1;
 //   median +=1;
 //   let mut resumm =2;
    let mut i;
    let mut an =Vec::<u128>::new();
    let mut bn =Vec::<u128>::new();
    loop {       
         i =0;
         loop {
              println!("loop! {} {}",a1[i],a1[i+1]);
              an.push((a1[i]+a1[i+1]) % modu);
              i +=1;  if i==median {i -=1; break;}
         }
     //*                                    
         println!("an! {}",i);
         an.push((2*an[i]+a1[i]) % modu);
         an.push((2*an[i+1]+a1[i+1]) % modu);
         bn.push((an[median]-a1[median]) % modu);
         println!("                ! {} {}",a1[median],b1[0]);
         bn.push((a1[median]-b1[0]) % modu);
         i =0;
         loop {
              if i>0 {     
                 println!("loop2 ended {} {}",b1[i],b1[i+1]);
                 bn.push((b1[i]-b1[i+1]) % modu);
              }
              i +=1;  if i==(median-1) {break;}
         }
         println!("loop2 ended");
                                    //*/
         a1=an.to_vec(); an.clear();
         median +=1;
         b1=bn.to_vec(); bn.clear();
         //Обчислення суми S(K)
         //resumm +=1;
         m +=1;  if m==k2.try_into().unwrap() {break;}
    }
    println!("! {} {} {}",m,median,k);
}
