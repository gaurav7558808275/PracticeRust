use rand::Rng;
use std::cmp::Ordering;

fn main() {


    
    //let mut gen = thread_rng();
    //let rdm = gen.gen_range(1,100);
    let rdm: u32 = rand::thread_rng().gen_range(1,100);
    println!("the random is {}",rdm);
    loop{
            println!("Enter a number!");
            let mut guess: String = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .expect("some problem");

                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
            //println!("The number guessed is {} ",guess);
            
            match guess.cmp(&rdm)
            {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal =>
                {
                    println!("You win!");
                    break;
                },
            }
    
        }
}