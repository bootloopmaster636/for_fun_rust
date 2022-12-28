use std::{
    io::{stdin, stdout, Write},
    sync::mpsc,
};

fn clearscreen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}

fn sleep_sort(array: Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::<i32>::new();
    let how_many = array.len();
    let (tx, rx) = mpsc::channel();

    for i in array {
        let tx = tx.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(i as u64));
            tx.send(i).unwrap();
        });
    }

    for _ in 0..how_many {
        sorted.push(rx.recv().unwrap());
    }
    sorted
}

fn main() {
    let mut temp_inp = String::new();
    let mut array: Vec<i32> = Vec::<i32>::new();

    clearscreen();
    println!("How many number you want to insert?");
    stdin()
        .read_line(&mut temp_inp)
        .expect("Failed to read num");
    let how_many = temp_inp.trim().parse::<i32>().expect("Not a number!");

    for i in 0..how_many {
        print!("Insert number {}: ", i + 1);
        stdout().flush().expect("Failed to flush stdout");
        temp_inp.clear();
        stdin()
            .read_line(&mut temp_inp)
            .expect("Failed to read num");
        let num = temp_inp.trim().parse::<i32>().expect("Not a number!");
        array.push(num);
    }

    let sorted = sleep_sort(array);
    println!("Sorted array: {:?}", sorted);
}
