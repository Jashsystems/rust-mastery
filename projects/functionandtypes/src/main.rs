fn level_of(line: &str) -> &'static str {
    if line.contains("ERROR") {
        "ERROR"
    } else if line.contains("WARN") {
        "WARN"
    } else {
        "INFO"
    }
}

fn main() {
    let text = "INFO start\nERROR disk full\nWARN slow response\nINFO user login\nERROR timeout\nINFO done";

    let mut errors: u32 = 0;
    let mut warnings: u32 = 0;
    let mut infos: u32 = 0;

    for line in text.lines() {
        let level = level_of(line);

        if level == "ERROR" {
            errors += 1;
        } else if level == "WARN" {
            warnings += 1;
        } else if level == "INFO" {
            infos += 1;
        }
    }

    println!("errors: {}", errors);
    println!("warnings: {}", warnings);
    println!("infos: {}", infos);
}