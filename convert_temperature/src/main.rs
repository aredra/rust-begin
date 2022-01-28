use std::io;

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.) * 5. / 9.
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 9. / 5. + 32.
}

fn main() {
    println!("Input \"exit\" to quit");

    'main: loop {
        println!("Input F(Fahrenheit) or C(Celsius) about your degree");

        let mut type_temp = String::new();
        loop {
            io::stdin().read_line(&mut type_temp)
                .expect("Failed to read line");
        
            type_temp = type_temp.trim().to_string();
            
            if type_temp == "exit" {
                break 'main;
            }

            if type_temp != "F" && type_temp != "f" 
                && type_temp != "C" && type_temp != "c" {
                type_temp = String::new();
                println!("Please input type F or C");
                continue;
            }
            break;
        }
    
        println!("Input your degree");
        loop {
            let mut degree = String::new();
            io::stdin().read_line(&mut degree)
                .expect("Failed to read line");                        println!("{}", type_temp);
            
            degree = degree.trim().to_string();
            
            if degree == "exit" {
                break 'main;
            }

            let degree: f64 = match degree.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input number!");
                    continue;
                }
            };
            
            if type_temp == "F" || type_temp == "f" {
                println!("{}F is {}C", degree, fahrenheit_to_celsius(degree));
            }
            else {
                println!("{}C is {}F", degree, celsius_to_fahrenheit(degree));
            }
            break;
        }
    }
}
