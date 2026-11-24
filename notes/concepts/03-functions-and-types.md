## Session 3: Functions and types

Right now all your logic sits inside `main`. Real programs split work into small named pieces called **functions**. Today we pull the "decide which level a line is" logic out of the loop.

### Concept 1: Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let total = add(2, 3);
    println!("{}", total);   // 5
}
```

Take it apart:

- `fn add` defines a function named `add`.
- `(a: i32, b: i32)` are the **parameters**: the inputs, each with a name and a type. Rust always requires the types here.
- `-> i32` says what type of value comes **out**.
- `a + b` has **no semicolon**. In Rust, the last expression in a function, without a `;`, is the value it returns. Adding a `;` there turns it into a statement that returns nothing, and the compiler will complain.
- `add(2, 3)` **calls** the function, and the result is stored in `total`.

### Concept 2: Types

Every value in Rust has a type, and the compiler checks that you use them correctly. The ones we need now:

| Type | Holds | Example |
|---|---|---|
| `i32` | whole numbers, positive or negative | `-5`, `42` |
| `u32` | whole numbers, zero or positive only | `0`, `7` |
| `bool` | true or false | `true` |
| `&str` | text | `"ERROR"` |

Rust usually guesses the type for you (`let n = 0;` becomes `i32`). You can also state it yourself: `let n: u32 = 0;`. Counters can never be negative, so `u32` is the honest choice for them.

**Try this experiment:** write `let n: u32 = -1;` and run it. Read the error. Rust refuses, because a `u32` can't hold a negative number.

### Concept 3: Returning text from a function

```rust
fn describe(n: i32) -> &str {
    if n > 0 {
        "positive"
    } else {
        "not positive"
    }
}
```

Two things to notice:

1. `if` itself can produce a value. Each branch ends with text and **no semicolon**, so the winning branch's text becomes the result of the `if`, which becomes the function's return value.
2. This version won't compile as written. Rust will complain about a missing *lifetime specifier*. That is a genuine concept with its own lesson on Day 20, so for now, write `-> &'static str` instead. It means "text that is written directly in the code and lasts for the whole program", which is exactly what our return values are.

### Your task

Refactor loglens. Fill in the blanks:

```rust
fn level_of(line: &str) -> &'static str {
    // if the line contains "ERROR" -> return "ERROR"
    // else if it contains "WARN"   -> return "WARN"
    // else                         -> return "INFO"
    // (no semicolons on the final values)
}

fn main() {
    let text = "INFO start\nERROR disk full\nWARN slow response\nINFO user login\nERROR timeout\nINFO done";

    let mut errors: u32 = 0;
    let mut warnings: u32 = 0;
    let mut infos: u32 = 0;

    for line in text.lines() {
        let level = level_of(line);
        // now count using `level`, not `line`
        // hint: you can compare text with ==, e.g. level == "ERROR"
    }

    println!("errors: {}", errors);
    println!("warnings: {}", warnings);
    println!("infos: {}", infos);
}
```

The output should be identical to before: `2 / 1 / 3`.

**Why do this?** The classification rule now lives in one named place. When we later add `DEBUG`, or a better parser, we change one function and `main` stays clean. That separation is what makes bigger programs manageable.

**Break it on purpose:** add a `;` after `"INFO"` in the `else` branch of `level_of`. Read the error and tell me what Rust says the function is returning now.

### WHAT TO WRITE IN MY NOTES

Create `notes/concepts/03-functions-and-types.md`:

- **Function:** `fn name(param: Type) -> ReturnType { ... }`
- **Return value:** the last expression, **with no `;`**. A `;` there causes a "mismatched types" error.
- **Call:** `name(value)`
- **Types:** `i32` (signed whole numbers), `u32` (non-negative), `bool`, `&str` (text)
- **Type annotation:** `let n: u32 = 0;`
- **`if` as a value:** each branch ends in a value with no `;`
- **`&'static str`:** text written in the code. Full explanation comes with lifetimes.
- **In loglens:** `level_of` is the classification rule, separated from the counting loop.

Add to the compiler-error notebook: *mismatched types (stray `;` at the end of a function)* and *`u32` can't hold negative numbers*.

Send me your completed `level_of` and `main`, plus the error from the `;` experiment. If you'd like, finish the earlier DEBUG task on top of this version as well. It's now a one-line change in `level_of` and one new counter, which shows why the refactor was worth it.