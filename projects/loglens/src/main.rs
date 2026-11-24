fn main() {
    let text = "INFO start\nERROR crash\nWARN slow";
    for line in text.lines() {
        println!("-> {}", line);
    }
}