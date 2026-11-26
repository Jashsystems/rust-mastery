
fn main() {
    println!("========== EXPERIMENT 1: COPY ==========");

    let original: i32 = 500;
    let duplicate = original;

    println!("original: {original}");
    println!("duplicate: {duplicate}");

    assert_eq!(original, duplicate);


    println!("\n========== EXPERIMENT 2: MOVE ==========");

    let original = String::from("Systems Programming");
    let moved = original;

    // println!("{original}"); // Uncomment and observe the error.
    println!("moved: {moved}");

    assert_eq!(moved, "Systems Programming");


    println!("\n========== EXPERIMENT 3: CLONE ==========");

    let original = String::from("Rust Engineering");
    let duplicate = original.clone();

    println!("original: {original}");
    println!("duplicate: {duplicate}");

    assert_eq!(original, duplicate);


    println!("\n========== EXPERIMENT 4: INDEPENDENCE ==========");

    let mut original = String::from("Hello");
    let duplicate = original.clone();

    original.push_str(" Rust");

    println!("original: {original}");
    println!("duplicate: {duplicate}");

    assert_eq!(original, "Hello Rust");
    assert_eq!(duplicate, "Hello");


    println!("\n========== EXPERIMENT 5: COPY IN TUPLES ==========");

    let first = (10, 20, true);
    let second = first;

    println!("first: {:?}", first);
    println!("second: {:?}", second);

    assert_eq!(first, second);


    println!("\n========== EXPERIMENT 6: NON-COPY TUPLE ==========");

    let first = (String::from("Rust"), 2026);
    let second = first;

    // println!("{:?}", first); // Uncomment and observe the error.
    println!("second: {:?}", second);


    println!("\n========== ALL EXPERIMENTS COMPLETED ==========");
}
