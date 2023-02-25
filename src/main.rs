// My frist program in Rust
// 2/24/2022
// Dominika Bobik

use std::io;

fn find_min(arr: &[i32]) -> i32{
    let mut curr_min = i32::MAX;
    for var in arr.iter() {
        if *var < curr_min {
            curr_min = *var;
        }
    }
    return curr_min;
}

fn find_max(arr: &[i32]) -> i32{
    let mut curr_max: i32 = i32::MIN;
    for var in arr.iter() {
        if *var > curr_max {
            curr_max = *var;
        }
    }
    return curr_max;
}

fn find_avg(arr: &[i32]) -> i32{
    let mut avg: i32 = 0;
    let mut count: i32 = 0;
    for var in arr.iter() {
        avg = avg + *var;
        count += 1;
    }
    avg = avg/count;
    return avg;
}

fn get_name() -> String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    return name;
}
fn main() {
    println!("Welcome to data analyzer! Enter your name:");
    let mut name = get_name();
    name.pop();
    println!("Hello {name}!");
    println!("Enter your 5 numbers:");
    let mut num_array:[i32; 5] = [0;5];
    let mut count:usize = 0;
    loop {
        let mut temp: String = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to get number!");
        let res: Result<i32, std::num::ParseIntError> = temp.trim().parse();
        match res {
            Ok(d) => {
                num_array[count] = d;
                count+=1;
            },
            Err(..) => {
                println!("Not a number!");
            }
        }
        if count >= 5 {
            break;
        }
    }

    for i in 0..5 {
        print!("arr[{i}] = {} ", num_array[i]);
    }
    print!("\n");
    println!("Max: {}", find_max(&num_array));
    println!("Min: {}", find_min(&num_array));
    println!("Avg: {}", find_avg(&num_array));
}
