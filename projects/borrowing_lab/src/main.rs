
// ============================================================
// PROJECT: BORROWING LAB
// DAY 3 — RUST FUNDAMENTALS
//
// Concepts:
// 1. References
// 2. Shared borrowing
// 3. Multiple immutable references
// 4. Mutable borrowing
// 5. Modifying data through references
// 6. Borrowing in functions
// 7. Sequential borrowing
// 8. Borrowing rules
// 9. String length
// ============================================================

// ------------------------------------------------------------
// 1. SHARED BORROWING
// ------------------------------------------------------------
// & creates a reference.
// A reference allows access to data without taking ownership.
// Shared references allow reading, not modifying.

fn show(s: &String) {
    println!("Borrowed value: {}", s);
}

fn shared_borrow_example() {
    let name = String::from("loglens");

    show(&name);

    // Ownership remains with name.
    println!("Original value: {}", name);
}

// ------------------------------------------------------------
// 2. MULTIPLE SHARED REFERENCES
// ------------------------------------------------------------
// Rust allows multiple shared references simultaneously.
// They can all read the same data.

fn multiple_shared_example() {
    let language = String::from("Rust");

    let first = &language;
    let second = &language;
    let third = &language;

    println!("First reference: {}", first);
    println!("Second reference: {}", second);
    println!("Third reference: {}", third);

    // All references are read-only.
}

// ------------------------------------------------------------
// 3. MUTABLE BORROWING
// ------------------------------------------------------------
// &mut creates a mutable reference.
// It allows a function to modify the original data.
// The original variable must be declared with let mut.

fn add_exclaim(s: &mut String) {
    s.push_str("!");
}

fn mutable_borrow_example() {
    let mut name = String::from("hi");

    add_exclaim(&mut name);

    println!("Modified string: {}", name);
}

// ------------------------------------------------------------
// 4. MODIFYING THROUGH A REFERENCE
// ------------------------------------------------------------
// A mutable reference can be used to modify existing data.

fn add_text(s: &mut String, text: &str) {
    s.push_str(text);
}

fn modify_example() {
    let mut message = String::from("Hello");

    add_text(&mut message, " Rust");

    println!("Updated message: {}", message);
}

// ------------------------------------------------------------
// 5. BORROWING IN FUNCTIONS
// ------------------------------------------------------------
// Functions can borrow data instead of taking ownership.
// This allows the caller to continue using the original value.

fn display_name(name: &String) {
    println!("Name: {}", name);
}

fn function_borrow_example() {
    let name = String::from("Jash");

    display_name(&name);

    // name is still valid.
    println!("Still accessible: {}", name);
}

// ------------------------------------------------------------
// 6. SEQUENTIAL BORROWING
// ------------------------------------------------------------
// Multiple shared borrows can happen together.
// A mutable borrow can happen after those shared references
// are no longer being used.

fn sequential_borrow_example() {
    let mut text = String::from("Rust");

    let first = &text;
    let second = &text;

    println!("Shared 1: {}", first);
    println!("Shared 2: {}", second);

    // The shared references are no longer used here.
    let mutable = &mut text;

    mutable.push_str(" Programming");

    println!("After modification: {}", mutable);
}

// ------------------------------------------------------------
// 7. COUNTING STRING LENGTH
// ------------------------------------------------------------
// &String borrows the String without taking ownership.
// -> usize specifies the return type.
// len() returns the number of bytes, not necessarily
// the number of Unicode characters.

fn count_chars(s: &String) -> usize {
    s.len()
}

fn count_example() {
    let name = String::from("loglens");

    let first_count = count_chars(&name);
    let second_count = count_chars(&name);

    println!("First count: {}", first_count);
    println!("Second count: {}", second_count);

    // name remains usable after both function calls.
    println!("Original string: {}", name);
}

// ------------------------------------------------------------
// 8. BORROWING RULE — COMPILE-TIME ERROR EXAMPLES
// ------------------------------------------------------------
// Rust does not allow conflicting shared and mutable borrows.
//
// ERROR E0502:
//
// let mut s = String::from("hi");
// let r1 = &s;
// let r2 = &mut s;
// println!("{} {}", r1, r2);
//
// Why?
// r1 is a shared reference.
// r2 is a mutable reference.
// They are both used while their borrows overlap.
//
// ERROR E0596:
//
// let name = String::from("hi");
// add_exclaim(&mut name);
//
// Why?
// name is immutable, so it cannot be mutably borrowed.
//
// These examples are commented out intentionally.
// Uncommenting them will prevent compilation.

// ------------------------------------------------------------
// 9. TEST CASES
// ------------------------------------------------------------

fn run_tests() {
    // Test 1: Shared borrowing
    let original = String::from("Rust");

    show(&original);

    assert_eq!(original, "Rust");

    // Test 2: Mutable borrowing
    let mut greeting = String::from("Hello");

    add_exclaim(&mut greeting);

    assert_eq!(greeting, "Hello!");

    // Test 3: Appending text
    let mut language = String::from("Rust");

    add_text(&mut language, " Programming");

    assert_eq!(language, "Rust Programming");

    // Test 4: Counting length
    let word = String::from("loglens");

    assert_eq!(count_chars(&word), 7);
    assert_eq!(count_chars(&word), 7);

    // Test 5: Original data remains usable
    assert_eq!(word, "loglens");

    println!("All borrowing tests passed!");
}

// ------------------------------------------------------------
// MAIN
// ------------------------------------------------------------

fn main() {
    println!("========== BORROWING LAB ==========\n");

    println!("--- Shared Borrowing ---");
    shared_borrow_example();

    println!("\n--- Multiple Shared References ---");
    multiple_shared_example();

    println!("\n--- Mutable Borrowing ---");
    mutable_borrow_example();

    println!("\n--- Modifying Through References ---");
    modify_example();

    println!("\n--- Function Borrowing ---");
    function_borrow_example();

    println!("\n--- Sequential Borrowing ---");
    sequential_borrow_example();

    println!("\n--- Counting String Length ---");
    count_example();

    println!("\n--- Tests ---");
    run_tests();

    println!("\nBorrowing lab completed!");
}
