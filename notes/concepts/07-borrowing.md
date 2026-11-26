Reference

What it means: a reference lets you use a value without owning it. Creating one is called borrowing.
Why Rust needs it: without references, every function call would take ownership and the caller couldn't use the value afterwards.
Syntax: &x (shared borrow), &mut x (mutable borrow). The parameter type is written s: &String or s: &mut String.
rust
fn count_chars(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("loglens");
    let a = count_chars(&name);
    let b = count_chars(&name);   // works, name was only lent
    println!("{} {} {}", a, b, name);
}

The two borrow types

	Shared &x	Mutable &mut x
Read	yes	yes
Change	no	yes
How many at once	any number	exactly one
Together with the other kind	no	no

The rule: at any moment you may have either many readers or one writer, never both. This prevents data from changing while someone is reading it.

Both places must say mut

rust
let mut name = String::from("hi");   // variable must be mut
add_exclaim(&mut name);              // borrow must be mut

Common compiler errors

Error	Cause	Fix
E0382, borrow of moved value	passed name instead of &name, so ownership moved	pass &name
E0596, cannot borrow as mutable	&mut on a variable not declared mut	let mut name
E0502, cannot borrow as mutable because it is also borrowed as immutable	a & borrow is still in use while a &mut is created	finish using the & first, or restructure

When to use

Default to & (read only).
Use &mut only when the function must change the data.
Pass by value (move) only when the function should keep or consume the data.

Practical improvement: prefer &str over &String for text parameters. A &str parameter accepts both string literals and borrowed Strings, so it's more flexible. Slices get their own lesson next.

In loglens: level_of(line: &str) borrows its input, so line is still usable after the call.

Compiler-error notebook entries
## E0382: borrow of moved value (missing &)
- Why it happened: show(name) moved the String into the function
- What Rust was enforcing: one owner at a time
- How I fixed it: show(&name)
- What I learned: pass a reference when the function only needs to look

## E0596: cannot borrow as mutable
- Why it happened: &mut on a variable declared without mut
- What Rust was enforcing: mutation must be declared at the variable and at the borrow
- How I fixed it: let mut name
- What I learned: mut appears twice so changes are never accidental

## E0502: mutable borrow while shared borrow is alive
- Why it happened: r1 (&s) was still in use when r2 (&mut s) was created
- What Rust was enforcing: many readers or one writer, never both
- How I fixed it: used r1 before creating r2, or dropped the idea
- What I learned: this stops data changing under a reader
Dashboard update
Introduced: ownership, move vs Copy, borrowing (& and &mut)
Written independently: count_chars with a reference
Compiler errors seen: E0382, E0596, E0502 (plus E0384, linker)
Next: slices and &str