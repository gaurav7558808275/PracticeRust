struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // simple fucntion practice

    fn simple_add(num1:u32 ,num2:u32) -> u32
    {
        return num1 + num2;
    }

    let x = simple_add(3,4);
    println!("{x}");

    // instantating a structur.
    struct Student{
        name:String,
        class:u16,
        divition:u16,
        marks:u16,
        school_name:String,
    } 

    let _x = Student{
        name:String::from("gaurav"), // this function allocated this data in heap
        class:10,
        divition:1,
        marks:80,
        school_name:String::from("blaala"),
    };
    let y = Student{
        name : String::from("kumar"),
        .._x // this copies all information from struct '_x'
    };

    // tuples type of struct
struct Blaa(i32,u32,i32);
let Xx = Blaa(1,2,3);
println!("{0}",Xx.0);


}

