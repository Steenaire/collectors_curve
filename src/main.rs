extern crate rand;

use rand::Rng;

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

}