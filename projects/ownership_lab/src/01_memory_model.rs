
fn main() {
    // ==========================================
    // EXPERIMENT 1: STACK
    // ==========================================

    let age = 18;
    let another_age = age;

    println!("Age: {}", age);
    println!("Another age: {}", another_age);

    // Both variables remain valid because i32 is Copy.


    // ==========================================
    // EXPERIMENT 2: HEAP
    // ==========================================

    let name = String::from("loglens");

    println!("Name: {}", name);
    println!("Length: {}", name.len());
    println!("Capacity: {}", name.capacity());


    // ==========================================
    // EXPERIMENT 3: OWNERSHIP TRANSFER
    // ==========================================

    let first = String::from("Rust");
    let second = first;

    println!("New owner: {}", second);

    // Uncomment this line and observe the error:
    // println!("Old owner: {}", first);


    // ==========================================
    // EXPERIMENT 4: INDEPENDENT CLONE
    // ==========================================

    let original = String::from("Ownership");
    let cloned = original.clone();

    println!("Original: {}", original);
    println!("Cloned: {}", cloned);

    assert_eq!(original, cloned);

    println!("\nAll experiments completed!");
}
