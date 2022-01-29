// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        match arg.as_ref() {
            "sum" => sum(),
            "double" => double(),
            _ => count(arg)
        }
    }
}

fn sum() {
    let mut sum = 0;
    for i in 7..=23 {
        sum += i;
    }
    
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x <= 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut counter = 1;
    loop {
        print!("{} ", arg);
        if counter == 8 {
            break;
        }
        counter += 1;
    }

    println!();
}
