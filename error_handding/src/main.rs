use std::{error, fs::{self, File}, io::{self, ErrorKind, Read}};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("Hello");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_plus() -> Result<String, io::Error> {
    let mut  username_file = File::open("Hllo")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    
}


fn read_username_from_file_pro() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("Hello")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_plus_pro() -> Result<String, io::Error> {
    fs::read_to_string("Hllo")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// ? 符号可以对Option以及Result使用
fn main() {
    let greeting_file_result = File::open("Hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Creat file error: {}", e),
            },
            other => panic!("Problem opening the file."),
            
        },
    };

    match read_username_from_file() {
        Ok(k) => println!("{k}"),
        Err(e) => println!("Error: {}", e),
    }

    match read_username_from_file_plus() {
        Ok(username) => println!("username is {}", username),
        Err(error) => println!("Read error: {}", error),
    }

    match read_username_from_file_pro() {
        Ok(username) => println!("username is {username}"),
        Err(error) => println!("Error is {error}"),
    }
     
    match read_username_from_file_plus_pro() {
        Ok(username) => println!("username is {username}"),
        Err(error) => println!("Error is {error}"),
    }

    match last_char_of_first_line("Hello wworld I am Kangkan") {
        Some(word) => println!("{word}"),
        None => println!("No words!"),
    }
}
