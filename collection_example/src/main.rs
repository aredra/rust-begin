use std::collections::HashMap;

fn main() {

    let temp_integers = vec![1,2,3,4,5,5,6,7,8,9,10];

    println!("Max: {}", match max(&temp_integers) {
        Some(max) => max.to_string(),
        None => "Vector is empty".to_string(),
    });


    println!("Min: {}", match min(&temp_integers) {
        Some(min) => min.to_string(),
        None => "Vector is empty".to_string(),
    });

    println!("Mean: {}", match mean(&temp_integers) {
        Some(mean) => mean.to_string(),
        None => "Vector is empty".to_string(),
    });

    println!("Median: {}", match median(&temp_integers) {
        Some(median) => median.to_string(),
        None => "Vector is empty".to_string(),
    });

    mode(&temp_integers);
    pig_latin();
    company_text_editor();
}

fn max(temp_vec: &Vec<i32>) -> Option<i32> {
    let mut max = None;
    for i in temp_vec {
        if max.is_none() {
            max = Some(*i);
        } else {
            if *i > max.unwrap() {
                max = Some(*i);
            }
        }
    }
    max
}

fn min(temp_vec: &Vec<i32>) -> Option<i32> {
    let mut min = None;
    for i in temp_vec {
        if min.is_none() {
            min = Some(*i);
        } else {
            if *i < min.unwrap() {
                min = Some(*i);
            }
        }
    }
    min
}

fn mean(temp_vec: &Vec<i32>) -> Option<f32> {
    if temp_vec.len() == 0 {
        return None;
    }

    let mut sum = 0.0;
    for i in temp_vec {
        sum += *i as f32;
    }

    Some(sum / temp_vec.len() as f32)
}

fn median(temp_vec: &Vec<i32>) -> Option<i32> {
    if temp_vec.len() == 0 {
        return None;
    }

    let mut cloned_vec = temp_vec.clone();
    cloned_vec.sort();

    Some(cloned_vec[cloned_vec.len() / 2])
}

fn mode(temp_vec: &Vec<i32>) {
    if temp_vec.len() == 0 {
        println!("Vector is empty");
        return;
    }
    
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max_count = 0;
    
    for i in temp_vec {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
        }
    }

    println!("Mode: {:?}", map.iter().filter(|&(_, v)| v == &max_count).map(|(k, _)| k).collect::<Vec<&i32>>());
}

fn pig_latin() {
    const VOWELS: &str = "aeiou";
    println!("Enter a word for Pig-Latin(only English): ");
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Failed to read line");
    
    if input_text.trim().is_empty() {
        println!("Input is empty");
        return;
    }

    let mut result = String::new();
    let input_text_length = input_text.len();
    let start_char = &input_text[0..1];

    if VOWELS.contains(start_char) {
        result.push_str(&input_text[..input_text_length-1]);
        result.push_str("-hay");
    } else {
        result.push_str(&input_text[1..input_text_length-1]);
        result.push_str("-");
        result.push_str(start_char);
        result.push_str("ay");
    }

    println!("Pig Latin: {}", result);
}

fn company_text_editor() {
    println!(">>>>>>>>> Company data text editor is running!!!!");
    println!(">>>>>>>>> Use 'Add {{employee_name}} to {{department_name}}' to insert");
    println!(">>>>>>>>> Use 'Remove {{employee_name}} to {{department_name}}' to delete");
    println!(">>>>>>>>> Use 'Show {{department_name}}' to select specific department");
    println!(">>>>>>>>> Use 'Show All' to select all");
    println!(">>>>>>>>> Use 'Exit' to quit this program");
    
    let mut company_db: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut command = String::new();

        println!("Enter your command: ");
        std::io::stdin().read_line(&mut command).expect("Failed to read line");
        if command.trim() == "Exit" {
            break;
        }
        if command.trim() == "Show All" {
            println!("{:?}", company_db);
        } else {
            let command_split: Vec<&str> = command.split(" ").collect();
            let command_type = command_split[0];

            if command_type == "Add" {
                let command_name = command_split[1];
                let command_department = command_split[3].trim();
                let mut employees = company_db.entry(command_department.to_string()).or_insert(Vec::new());
                employees.push(command_name.to_string());
            } else if command_type == "Remove" {
                let command_name = command_split[1];
                let command_department = command_split[3].trim();
                let mut employees = company_db.entry(command_department.to_string()).or_insert(Vec::new());
                employees.retain(|x| x != command_name);
            } else if command_type == "Show" {
                let command_department = command_split[1].trim();
                let employees = company_db.get(command_department);
                if employees.is_some() {
                    println!("{:?}", employees.unwrap());
                } else {
                    println!("No employees in this department");
                }
            } else {
                println!("Invalid command");
            }
        }
    }
}