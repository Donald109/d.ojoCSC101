fn main() {
    //Write all parameters
    let p: f64 =510000.0;
    let r: f64 = 5.0;
    let t: f64 = 3.0;

    //Write all equations/functions that will be used
    let a = p*(1.0-(r/100.0)).powf(t);
    println!("The price of the TV has reduced to {:.2}₦ over 3 years.", a);

    let d = p-a;
    println!("The price of the TV reduced by {:.2}₦",d);
    //Assignment completed successfully

}