fn main() {
    let mut x = 5;
    let y = 5;
    let y = y + 1;
    println!("The value of x is :{x}");
    x = 6;
    println!("The value of x is: {x}");

    {
        let y = y * 2;
        println!("The value of x in the inner scrope is:{y}")
    }
    println!("The value of x is:{y}");
}
