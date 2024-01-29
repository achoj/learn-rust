fn main() {
    let hello = "Здравствуйте";

    let hello2 = "Hello world";

    let hello3 = "नमस्ते";

    let s = 'd';

    println!("{}", hello.len());

    println!("{}", hello2.len());

    println!("{}", hello3.len());

    let a = hello;

    for i  in a.as_bytes() {
        println!("{i}");
    }


}
