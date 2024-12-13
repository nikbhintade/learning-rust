use std::io;

// `#[allow(dead_code)]` attribute suppresses warnings for unused functions, structs, or variables in the code.
#[allow(dead_code)]
pub fn greet() {
    println!("Please Enter Your Name:");
    let mut name = String::new();

    loop {
        io::stdin()
            .read_line(&mut name)
            .expect("failed to read input");

        // name<String> -> after trim, <&str> -> after to_uppercase, <String> (but match statment is comparing it to <&str> type so we need to convert it) -> as_str <&str>
        match name.trim().to_uppercase().as_str() {
            "" => {
                println!("You forgot to enter your name, please enter it again");
                continue;
            }
            "Help" => println!("You're an idiot so bye, that's the only greeting you're gonna get"),
            // if you don't use .trim() on input it will have newline character and will push next part on new line
            _ => println!("Hi {}, nice to meet you", name.trim()),
        }
        break;
    }
}
