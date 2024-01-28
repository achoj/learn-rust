fn main() {

    // let vec1: Vec<u32> = Vec::new();
    let vec2 = vec![1, 2 ,3];

    let mut vec3 = Vec::new();
    vec3.push(12);
    vec3.push(23);

    let third: Option<&i32> = vec3.get(0);
    
    match third {
        Some(third) => println!("The value is {third}"),
        None => println!("No this value"),
    }

    for i in &vec2 {
        println!("{i}");
    }

    for i in &mut vec3 {
        *i += 50;
        println!("{i}");
    }

    for i  in &vec3 {
        println!("{i}");
    }

    let ii = &vec3[0];
    let iii = &mut vec3[0];

    let rr = &mut vec3[0];

    // println!("{rr}, {iii}");

    let data = "Hello world";
    let data = "hello".to_string();


}
