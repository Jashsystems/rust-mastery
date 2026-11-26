# Borrowing and References in Rust

**Day:** 03  
**Topic:** Borrowing  
**Status:** Practiced and implemented  
**Project:** `projects/borrowing_lab`

## 1. What is Borrowing?

Borrowing allows a function or variable to access data without taking ownership of it.

Rust uses references for borrowing.

```rust
let name = String::from("loglens");

let reference = &name;

println!("{}", reference);
println!("{}", name);
```

The original variable remains the owner.

## 2. Shared Borrowing (`&`)

A shared reference allows reading data without modifying it.

```rust
fn show(s: &String) {
    println!("{}", s);
}

fn main() {
    let name = String::from("loglens");

    show(&name);

    println!("{}", name);
}
```

Both the function and the original owner can access the String.

### Multiple Shared References

Rust allows multiple shared references at the same time.

```rust
let name = String::from("Rust");

let first = &name;
let second = &name;

println!("{}", first);
println!("{}", second);
```

Shared references are read-only.

## 3. Mutable Borrowing (`&mut`)

A mutable reference allows modification of borrowed data.

```rust
fn add_exclaim(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut name = String::from("hi");

    add_exclaim(&mut name);

    println!("{}", name);
}
```

Output:

```text
hi!
```

Two things are important:

- `let mut name` allows the variable to be modified.
- `&mut name` creates a mutable reference.

## 4. Modifying Data Through References

Mutable references allow functions to modify the original data.

```rust
fn add_text(s: &mut String, text: &str) {
    s.push_str(text);
}

fn main() {
    let mut message = String::from("Hello");

    add_text(&mut message, " Rust");

    println!("{}", message);
}
```

Output:

```text
Hello Rust
```

No ownership transfer occurs.

## 5. Borrowing in Functions

Functions can accept references instead of taking ownership.

```rust
fn display_name(name: &String) {
    println!("{}", name);
}

fn main() {
    let name = String::from("Jash");

    display_name(&name);

    println!("{}", name);
}
```

The String remains usable after the function call.

## 6. Sequential Borrowing

A mutable borrow can be created after shared references are no longer being used.

```rust
fn main() {
    let mut text = String::from("Rust");

    let first = &text;
    let second = &text;

    println!("{}", first);
    println!("{}", second);

    let mutable = &mut text;

    mutable.push_str(" Programming");

    println!("{}", mutable);
}
```

Rust's borrow checker ensures conflicting references are not used simultaneously.

## 7. The Borrowing Rules

Rust follows one fundamental rule:

**At any given time, you can have either:**

- Any number of shared references (`&T`).
- Exactly one mutable reference (`&mut T`).

You cannot have an active mutable reference alongside active shared references to the same data.

This prevents data races and unexpected changes while data is being read.

## 8. Common Borrowing Errors

### E0502 — Conflicting Borrows

```rust
let mut s = String::from("hi");

let r1 = &s;
let r2 = &mut s;

// println!("{} {}", r1, r2);
```

**Reason:** A shared reference and a mutable reference cannot be used in conflicting overlapping borrows.

### E0596 — Cannot Mutably Borrow

```rust
let name = String::from("hi");

// add_exclaim(&mut name);
```

**Reason:** The variable was declared immutable using `let`, so it cannot be mutably borrowed.

## 9. String Length and `usize`

References can be passed to functions that return values.

```rust
fn count_chars(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("loglens");

    let first = count_chars(&name);
    let second = count_chars(&name);

    println!("{}", first);
    println!("{}", second);

    println!("{}", name);
}
```

Output:

```text
7
7
loglens
```

**Important:** `String::len()` returns the number of bytes, not necessarily the number of Unicode characters.

`usize` is an unsigned integer type commonly used for sizes and indexing.

## 10. Shared vs Mutable Borrowing

| Feature | Shared (`&`) | Mutable (`&mut`) |
|---|---|---|
| Read data | Yes | Yes |
| Modify data | No | Yes |
| Multiple simultaneous references | Yes | No |
| Ownership transfer | No | No |

## 11. Practical Implementation

**Project:** `borrowing_lab`

Location: `projects/borrowing_lab/src/main.rs`

Implemented:

- Shared borrowing
- Multiple shared references
- Mutable borrowing
- Modifying data through references
- Borrowing in functions
- Sequential borrowing
- String length calculation
- Borrowing error explanations
- Assertion-based checks

## 12. Important Takeaways

- `&` creates a shared reference.
- `&mut` creates a mutable reference.
- Borrowing does not transfer ownership.
- Multiple shared references are allowed.
- Only one mutable reference can be active at a time.
- Shared and mutable references cannot have conflicting overlapping use.
- Borrowing makes functions more flexible without unnecessary ownership transfers.

**Engineering principle:** Borrow data when you only need temporary access. Use mutable borrowing when modification is necessary.

---

**Next concept:** Slices and String Slices.