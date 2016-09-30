fn main() {
    let lines = "aggaggt\naggaggt\ngggacgt\ngggacgt\ngggactt".lines();

    let mut data_vector = vec![];

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
        data_vector.push(line);
    }

    for x in 0..data_vector.len() {
        println!("{}", data_vector[x]);
    }

}