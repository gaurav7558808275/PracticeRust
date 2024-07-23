
fn main() {

    println!("Enter the number!");
    println!("{0} is better than {1}","alice","alphonse");
    
    println!("{a} {b} {c}", b = "hey" , c = "heyy" , a = "heyyy" );

    println!("{:b}",23);

    // can add padding
    println!("{num:0>5}",num = 6);
    println!("{num:0<5}",num = 6);

    //another usage
    let wdt:usize = 6;
    let num:u32 = 1;
    println!("{num:0>wdt$}");

    let num:f64 = 3.1415674564;
    println!("value of pi: {num:.3}");



}
