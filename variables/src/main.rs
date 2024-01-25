
fn main() {


    // add some comments.
    println!("---Learn mut variables---");
    let x = 10;
    println!("The value of x is {x}");

    let x = 100;
    println!("The value of x is {x}");

    println!();
    println!("---Learn shadowing---");
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is : {y}");   
    }

    println!("The value of y is: {y}");
}
