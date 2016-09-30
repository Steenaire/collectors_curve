fn main() {
    let data = ["aggaggt","aggaggt","gggacgt","gggacgt","gggactt"];
    //println!("The array has {} elements", data.len());

    //let mut times = data.len();
    let mut x = 0;
    let mut done = false;

    while !done {
        println!("The sequence is {}", data[x]);
        x += 1;

        if x == data.len() {
            done = true;
        }
    }
}