fn main() {
    let text = "INFO start\nERROR disk full\nWARN slow response\nINFO user login\nERROR timeout\nINFO done";

    let mut errors = 0;
    let mut warnings = 0;
    let mut infos = 0;

    for line in text.lines() {
        if line.contains("ERROR") {
            errors += 1;
        } else if line.contains("WARN") {
            warnings += 1;
        } else {
            infos += 1;
        }
    }

    println!("errors: {}", errors);
    println!("warnings: {}", warnings);
    println!("infos: {}", infos);
}