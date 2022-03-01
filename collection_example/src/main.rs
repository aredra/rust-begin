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