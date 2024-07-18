


fn main()
{
    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");
    const BLA:u32 = 100000;
    println!("{BLA}");


    // shadowing
    let y = 10;
    println!("{y}"); 
    let y = 11; 
    println!("{y}"); 
    let y: char = '😻';
    println!("{y}"); 


    // tuple in RUST - Compound type

    let tup: (u32,i32,f32) = (1,-5,5.6);
    let (a,b,c) = tup;
    // printed in 2 ways
    println!("{a} {b} {c}");
    println!("{} {} {}",tup.0,tup.1,tup.2);

    // arrays

    let ar = [1,2,3,4,5];
    
    let ar: [u32; 6] = [1,2,3,4,5,6];

    let ar = [3; 5]; // [3,3,3,3,3] for repetative array inputs

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess)
    .ok()
    .expect("failed to read line");

}
