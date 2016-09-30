extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let lines = "aggaggt\naggaggt\ngggacgt\ngggacgt\ngggactt".lines();

    let mut data_vector = vec![];

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
        data_vector.push(line);
    }

    //before shuffle
    for x in 0..data_vector.len() {
        println!("{}", data_vector[x]);
    }

    //after shuffle
    rand::thread_rng().shuffle(&mut data_vector);

    println!("{:?}", data_vector);

    let mut rarefaction = HashMap::new();
    let mut log_hash = HashMap::new();

    for x in 0..data_vector.len() {
        if rarefaction.contains_key(data_vector[x]) {
            *rarefaction.get_mut(data_vector[x]).unwrap() += 1;
            log_hash.insert(x, 0);
        } else {
            rarefaction.insert(data_vector[x], 1);
            log_hash.insert(x, 1);
        }
    }

    for (sample, unique_status) in &log_hash {
        println!("{}: {}", sample, unique_status);
    }

}