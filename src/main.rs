use std::fs;
fn main() {
    let contents = fs::read_to_string("collected.txt")
        .expect("Error reading file");
    //println!("{}", contents)
    let recorded_ips: Vec<&str> = contents.lines().collect();
    println!("{:?}", recorded_ips)
}
