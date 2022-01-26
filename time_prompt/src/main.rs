use std::time::{SystemTime};
use std::io::Write;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Failed to read line");

    return line.trim().to_string()
}

fn main() {
    loop {
        let input = prompt("> ");
        if input == "now" {
            let unixtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            println!("Current Unix time is {:?}", unixtime);
        }
        else if input == "exit" {
            break;
        }
    }
}
