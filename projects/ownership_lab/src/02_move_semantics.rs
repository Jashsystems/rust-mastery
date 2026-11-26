
fn main() {
    println!("=== EXPERIMENT 1: STRING MOVE ===");

    let first = String::from("Rust Engineering");

    println!("Before move:");
    println!("first = {}", first);

    let second = first;

    println!("\nAfter move:");
    // println!("first = {}", first); // Uncomment to see the compiler error.
    println!("second = {}", second);

    println!("\n=== EXPERIMENT 2: INTEGER COPY ===");

    let a = 100;
    let b = a;

    println!("a = {}", a);
    println!("b = {}", b);

    println!("\n=== EXPERIMENT 3: MOVE INTO FUNCTION ===");

    let language = String::from("Rust");

    consume(language);

    // println!("{}", language); // Uncomment to see the compiler error.

    println!("\n=== EXPERIMENT 4: RETURN OWNERSHIP ===");

    let original = String::from("Ownership");
    let recovered = return_ownership(original);

    println!("Recovered value: {}", recovered);
}

fn consume(value: String) {
    println!("Function owns: {}", value);
}

fn return_ownership(value: String) -> String {
    value
}
