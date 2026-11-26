Code 1: the move
rust
let a = String::from("hello");
let b = a;
println!("{}", b);   // ok
println!("{}", a);   // ERROR E0382
Concept: move. Ownership of the text transfers from a to b.
What it means: after let b = a;, only b owns the text. a is dead and can't be used.
Why Rust needs it: if a and b both owned the text, both would release it at the end of main, which releases the same memory twice (a double free). One owner at a time makes that impossible.
Error: E0382: borrow of moved value: a
Fixes:
let b = a.clone(); makes a real second copy, so both are valid. It costs extra memory and time.
let b = &a; makes b borrow the text without owning it. This is the next concept.
Use when: use .clone() only when you truly need two independent copies. Prefer borrowing.
In loglens: LogEntry will own its String text, so passing an entry around will move it.
Code 2: the copy
rust
let x = 5;
let y = x;
println!("{} {}", x, y);   // ok, both valid
Concept: Copy types.
What it means: simple fixed-size values are duplicated automatically on assignment, so both variables stay valid.
Why Rust allows it: duplicating a number is trivial and has no memory to release, so no ownership problem exists.
Rule: a type is copied only if it implements Copy. Examples: i32, u32, bool, char, tuples and arrays of Copy types. String does not implement Copy.
In loglens: our counters (u32) are copied freely.
Your table, for the notes
	String	i32
Holds data on	heap	stack
Assignment	move	copy
Original valid afterwards?	no	yes
Implements Copy?	no	yes

Compiler-error notebook entry:

## E0382: borrow of moved value
- Why it happened: assigned a String to a new variable, then used the old one
- What Rust was enforcing: one owner at a time
- How I fixed it: clone() or borrow with &
- What I learned: Copy vs move depends on the Copy trait