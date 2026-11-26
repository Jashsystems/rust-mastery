# Ownership in Rust

**Day:** 03  
**Topic:** Ownership  
**Status:** Practiced and implemented  
**Project:** `projects/ownership_lab`

## 1. What is Ownership?

Ownership is Rust's system for managing memory safely without requiring a garbage collector.

Every value in Rust has an owner.

### Three fundamental rules

1. Every value has one owner at a time.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

## 2. Move Semantics

When a heap-allocated value such as `String` is assigned to another variable, ownership moves.

```rust
let first = String::from("Rust");
let second = first;

println!("{}", second);

// println!("{}", first); // ERROR: value was moved
```

**Why?**

Ownership transfers from `first` to `second`. The original variable becomes invalid.

This prevents problems such as double-free memory errors.

## 3. Copy Semantics

Types implementing the `Copy` trait are copied instead of moved.

```rust
let x = 10;
let y = x;

println!("{}", x);
println!("{}", y);
```

Both variables remain valid because integers implement `Copy`.

Common Copy types include:

- Integers
- Floating-point numbers
- Booleans
- Characters
- Tuples containing only Copy types

## 4. Cloning

`clone()` creates an independent copy of a value.

```rust
let first = String::from("Rust");
let second = first.clone();

println!("{}", first);
println!("{}", second);
```

Both variables own separate String data.

Unlike Copy, cloning can involve allocating memory and copying data.

## 5. Scope and Automatic Cleanup

A variable remains valid within its scope.

```rust
fn main() {
    let outer = String::from("Outside");

    {
        let inner = String::from("Inside");
        println!("{}", inner);
    }

    println!("{}", outer);
}
```

When the inner scope ends, `inner` is automatically dropped.

Rust handles memory cleanup through its ownership and drop mechanisms.

## 6. Ownership in Functions

Passing a String by value transfers ownership to the function.

```rust
fn take_ownership(value: String) {
    println!("{}", value);
}

fn main() {
    let message = String::from("Hello Rust");

    take_ownership(message);

    // message is no longer valid here.
}
```

The function becomes the owner of the String.

## 7. Returning Ownership

Functions can return ownership to their callers.

```rust
fn give_ownership() -> String {
    let message = String::from("Rust");

    message
}

fn main() {
    let received = give_ownership();

    println!("{}", received);
}
```

The caller becomes the new owner of the returned String.

## 8. Key Differences

| Feature | Move | Copy | Clone |
|---|---|---|---|
| Ownership transfer | Yes | No | No |
| Original remains valid | No | Yes | Yes |
| Creates independent data | No | Yes, for copied value | Yes |
| Explicit operation | Assignment | Assignment | `.clone()` |

## 9. Practical Implementation

**Project:** `ownership_lab`

Location: `projects/ownership_lab/src/main.rs`

Implemented:

- Ownership rules
- Move semantics
- Copy semantics
- Cloning
- Scope and automatic cleanup
- Passing ownership to functions
- Returning ownership
- Assertion-based checks

## 10. Important Takeaways

- Every value has one owner.
- Assigning a String can move ownership.
- Copy types retain their original values after assignment.
- Clone creates an independent copy.
- Functions can take or return ownership.
- Rust automatically drops values when their owners leave scope.

**Engineering principle:** Ownership determines who is responsible for the lifetime of data.

---

**Next concept:** Borrowing and References.