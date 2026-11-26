
// ============================================================
// PROJECT: OWNERSHIP LAB
// DAY 3 — RUST FUNDAMENTALS
//
// Concepts:
// 1. Ownership rules
// 2. Move semantics
// 3. Copy semantics
// 4. Cloning
// 5. Scope and automatic memory cleanup
// 6. Ownership in functions
// 7. Returning ownership
// ============================================================

// ------------------------------------------------------------
// 1. OWNERSHIP
// ------------------------------------------------------------
// Every value in Rust has an owner.
// A value can have only one owner at a time.
// When the owner goes out of scope, the value is dropped.

fn ownership_example() {
    let name = String::from("Rust");

    println!("Owner: {}", name);

    // name owns the String.
}

// ------------------------------------------------------------
// 2. MOVE SEMANTICS
// ------------------------------------------------------------
// Assigning a String to another variable moves ownership.
// The original variable becomes invalid.
//
// let first = String::from("Ownership");
// let second = first;
//
// println!("{}", second); // Works
// println!("{}", first);  // ERROR: value was moved
//
// Only second owns the String after the move.

fn move_example() {
    let first = String::from("Ownership");
    let second = first;

    println!("New owner: {}", second);

    // first cannot be used here because ownership moved.
}

// ------------------------------------------------------------
// 3. COPY SEMANTICS
// ------------------------------------------------------------
// Simple types such as integers implement the Copy trait.
// Assignment copies their value instead of moving ownership.

fn copy_example() {
    let first = 100;
    let second = first;

    println!("First integer: {}", first);
    println!("Second integer: {}", second);

    // Both variables remain valid.
}

// ------------------------------------------------------------
// 4. CLONING
// ------------------------------------------------------------
// clone() creates an independent copy of a String.
// Both variables own separate String data.

fn clone_example() {
    let first = String::from("Rust Programming");
    let second = first.clone();

    println!("Original: {}", first);
    println!("Cloned: {}", second);

    // Both variables remain valid.
}

// ------------------------------------------------------------
// 5. SCOPE AND AUTOMATIC CLEANUP
// ------------------------------------------------------------
// A variable is valid within its scope.
// When the scope ends, Rust automatically drops its value.

fn scope_example() {
    let outer = String::from("Outside");

    {
        let inner = String::from("Inside");

        println!("Outer: {}", outer);
        println!("Inner: {}", inner);

        // inner is dropped when this block ends.
    }

    println!("Outer still exists: {}", outer);

    // inner cannot be accessed here.
}

// ------------------------------------------------------------
// 6. PASSING OWNERSHIP TO FUNCTIONS
// ------------------------------------------------------------
// Passing a String by value moves ownership into the function.
// The function becomes the new owner.

fn take_ownership(value: String) {
    println!("Function owns: {}", value);
    // value is dropped when this function ends.
}

fn function_ownership_example() {
    let message = String::from("Hello Rust");

    take_ownership(message);

    // message cannot be used here because ownership moved.
}

// ------------------------------------------------------------
// 7. RETURNING OWNERSHIP FROM FUNCTIONS
// ------------------------------------------------------------
// A function can return ownership to its caller.

fn give_ownership() -> String {
    let message = String::from("Ownership returned");

    message
}

fn return_ownership_example() {
    let message = give_ownership();

    println!("Received: {}", message);

    // message now owns the returned String.
}

// ------------------------------------------------------------
// 8. TEST CASES
// ------------------------------------------------------------
// Assertions verify that our expected values are correct.
// If an assertion fails, the program panics.

fn run_tests() {
    // Test 1: Copy
    let x = 10;
    let y = x;

    assert_eq!(x, 10);
    assert_eq!(y, 10);

    // Test 2: Clone
    let original = String::from("Rust");
    let cloned = original.clone();

    assert_eq!(original, "Rust");
    assert_eq!(cloned, "Rust");

    // Test 3: Returned ownership
    let returned = give_ownership();

    assert_eq!(returned, "Ownership returned");

    println!("All ownership tests passed!");
}

// ------------------------------------------------------------
// MAIN
// ------------------------------------------------------------

fn main() {
    println!("========== OWNERSHIP LAB ==========\n");

    println!("--- Ownership ---");
    ownership_example();

    println!("\n--- Move ---");
    move_example();

    println!("\n--- Copy ---");
    copy_example();

    println!("\n--- Clone ---");
    clone_example();

    println!("\n--- Scope ---");
    scope_example();

    println!("\n--- Function Ownership ---");
    function_ownership_example();

    println!("\n--- Return Ownership ---");
    return_ownership_example();

    println!("\n--- Tests ---");
    run_tests();

    println!("\nOwnership lab completed!");
}
