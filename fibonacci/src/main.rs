use std::io::Write;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Failed to read line");

    return line.trim().to_string()
}

fn fibonacci(num: u32) -> u32{
    // let mut a = 1;
    // let mut b = 1;
    // let mut index = 2;

    // while index < num {
    //     let temp = a;
    //     a = b;
    //     b += temp;
    //     index += 1;
    // }

    // return b;

    if num < 3 {
        return 1;
    }
    
    return fibonacci(num - 1) + fibonacci(num - 2);
}

fn main() {
    println!("Please input number of Fibonacci");


    let num = loop {
        let num = prompt("> ");
        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number!");
                continue;
            }
        };

        break num;
    };

    println!("Result: {:?}", fibonacci(num));
}
