fn main() {
//List all parameters
    let t: f64 = 450000.0; //2
    let m: f64 = 1500000.0; //1
    let h: f64 = 750000.0; //3
    let d: f64 = 2850000.0; //3
    let a: f64 = 250000.0; //1
    let n: f64 = 10.0; //1+2+1+3+3
   
    //list out all equations
    let s = t + m + h + d + a;
    println!("Sum of all amounts is {}",s);
   //s = sum of all amounts
    let a = s/n;
    println!("Average of total sum is {}",a);
   //a =average of total sum 

   //Average of individual variables sepertely
   let am = m/(2.0);
   println!("Average of m is {}", am);
   let ah = h/3.0;
   println!("Average of h is {}", ah);
   let ad = d/3.0;
   println!("Average of d is {}", ad);
   let aa = a/(1.0);
   println!("Average of a is {}", aa);
   let at = a/(1.0);
   println!("Average of t is {}", at);


}