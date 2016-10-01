extern crate rand;

use std::io;
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter};
use std::path::Path;

fn main() {
    println!("Enter the path to the file of sequences:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let path = Path::new(input.trim());
    let display = path.display();
    // let lines = "aggaggt\naggaggt\ngggacgt\ngggacgt\ngggactt".lines();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("Rarefaction curve:"),
    }

    let lines = file_string.lines();

    let mut data_vector = vec![];

    for line in lines {
        data_vector.push(line);
    }

    //after shuffle
    rand::thread_rng().shuffle(&mut data_vector);

    let mut rarefaction = HashMap::new();
    let mut log_vector = vec![];

    //Sample the entire list of sequences, and if unique log it as 1, if not log it as 0
    for x in 0..data_vector.len() {
        if rarefaction.contains_key(data_vector[x]) {
            *rarefaction.get_mut(data_vector[x]).unwrap() += 1;
            log_vector.push(0);
        } else {
            rarefaction.insert(data_vector[x], 1);
            log_vector.push(1);
        }
    }

    let mut running_total = 0;

    
    //Create a new file one directory up and put the results of the curve in there    
    let f = File::create("../rarefaction_curve.txt").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    let mut data = String::new();


    for x in 0..log_vector.len() {
        if x == 0 {
            data = format!("Sample#\tUnique#\n{}\t{}\n", x+1, log_vector[x]);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("{}: {}", x+1, log_vector[x]);
        } else {
            running_total += log_vector[x];
            for y in 0..x {
                running_total += log_vector[y];
            }
            data = format!("{}\t{}\n", x+1, running_total);
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("{}: {}", x+1, running_total);
            running_total = 0;
        }
    }

}