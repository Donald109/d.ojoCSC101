fn main() {
    //Write out the parameters for each variable
    let p: f64 = 520000000.0;
    let r: f64 = 10.0;
    let y: f64 = 5.0;

    let a = p * (1.0 + (r / 100.0)).powf(y);
    println!("Amount is {}", a);
    let ci = a - p;
    println!("Compound Intrest is {}", ci);


}