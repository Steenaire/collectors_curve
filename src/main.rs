extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let lines = "aggaggt\naggaggt\ngggacgt\ngggacgt\ngggactt".lines();

    let mut data_vector = vec![];

    for (linenumber, line) in lines.enumerate() {
        // println!("{}: {}", linenumber, line);
        data_vector.push(line);
    }

    //before shuffle
    // for x in 0..data_vector.len() {
    //     println!("{}", data_vector[x]);
    // }

    //after shuffle
    rand::thread_rng().shuffle(&mut data_vector);

    println!("{:?}", data_vector);

    let mut rarefaction = HashMap::new();
    let mut log_vector = vec![];
    // let mut log_hash = HashMap::new();

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

    for x in 0..log_vector.len() {
        if x == 0 {
            println!("{}: {}, {}", x+1, log_vector[x], log_vector[x]);
        } else {
            running_total += log_vector[x];
            for y in 0..x {
                running_total += log_vector[y];
                // println!("{:?}", running_total);
            }
            println!("{}: {}, {}", x+1, running_total, log_vector[x]);
            running_total = 0;
        }
    }

}