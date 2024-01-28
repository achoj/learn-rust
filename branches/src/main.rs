

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }

    let ifvalue = if number < 5 {
        32
    } else {
        12
    };
    println!("{ifvalue}");
    

    // loop {
    //     println!("Hello again.")
    // }
    

    let mut count = 0;
    'counting_up: loop {
        println!("entering counting_up.");
        println!("the value of count is {count}");
        

        let mut remain = 10;
        println!("the value of remain is {remain}");
        loop {
            println!("entering new loop.");
            if remain == 0 {
                println!("Hello world");
                break;
            }
            if count == 2 {
               break 'counting_up; 
            }
            remain -= 1;
        }
        count += 1;
    }

    let a = [1,2,3,4,5,6];
    // more safe and quick than while.
    for element in a {
        println!("value of a is: {element}");
    }

    for number in (1..9).rev() {
       println!("{number}!"); 
    }
    println!("LIFEOFF.");
}
