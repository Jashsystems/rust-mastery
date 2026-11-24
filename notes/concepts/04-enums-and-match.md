Your code is correct, and it's the first real Rust you've written. I checked it against the expected output, and `level_of` and the counting loop do what they should (`2 / 1 / 3`). You also put the counters before the loop and used `u32`, both for the right reasons.

## One weakness to see

Look at this line: `if level == "ERROR"`. Now imagine you typo it as `"ERORR"`. Rust compiles it happily, the program runs, and your error count is silently 0. The compiler can't help, because to Rust it's just some text being compared to other text. The level names are "stringly typed", and bugs like that hide for months.

Rust's fix is to let you **define your own type** that can only hold the valid levels. That's today's concept.

## Session 4: Enums and `match`

### Concept 1: Enum

An **enum** is a type with a fixed list of possible values.

```rust
enum LogLevel {
    Error,
    Warn,
    Info,
}
```

- `LogLevel` is now a type, like `u32` or `bool`, but you designed it.
- A `LogLevel` value is exactly one of `Error`, `Warn` or `Info`. Nothing else exists.
- You write a value as `LogLevel::Error` (type name, two colons, variant name).
- A typo like `LogLevel::Erorr` is a **compile error**, so the bug from above becomes impossible.

### Concept 2: `match`

`match` looks at a value and runs the arm that fits it:

```rust
let level = LogLevel::Warn;

match level {
    LogLevel::Error => println!("bad"),
    LogLevel::Warn => println!("careful"),
    LogLevel::Info => println!("fine"),
}
```

- Each **arm** has the form `pattern => what to do,`
- The whole `match` can also produce a value, just like `if`:

```rust
let label = match level {
    LogLevel::Error => "E",
    LogLevel::Warn => "W",
    LogLevel::Info => "I",
};
```

### Concept 3: Exhaustiveness (the important part)

`match` must cover **every** variant. Delete the `Info` arm and run it. Rust refuses to compile and tells you exactly which case you forgot. Later, when you add `Debug` to the enum, the compiler will point at every `match` in your program that needs updating. You can't forget one, which is the reason many Rust programmers love enums.

### Your task

Change loglens to use the enum:

```rust
enum LogLevel {
    // three variants: Error, Warn, Info
}

fn level_of(line: &str) -> LogLevel {
    // same if / else if / else shape as before,
    // but return LogLevel::Error, LogLevel::Warn, LogLevel::Info
    // instead of text
}

fn main() {
    // same text and three counters as before

    for line in text.lines() {
        let level = level_of(line);

        // replace the if / else if chain with a match on `level`
        // each arm adds 1 to the right counter
    }

    // same three println! lines
}
```

The output must stay `2 / 1 / 3`.

**Break it on purpose, twice:**

1. Remove one arm from your `match`. Read the error and tell me what Rust says is missing.
2. Write `LogLevel::Erorr` somewhere. See how a typo is now caught immediately, unlike the text version.

**Stretch:** add a fourth variant `Debug`. Without changing anything else, run `cargo run` and see what the compiler tells you to fix. Then fix it by adding the `DEBUG` check in `level_of`, a counter, and a match arm.

### WHAT TO WRITE IN MY NOTES

Create `notes/concepts/04-enums-and-match.md`:

- **Enum:** `enum Name { A, B, C }` is a type with a fixed set of values.
- **Use a value:** `Name::A`
- **`match`:** `match value { Name::A => ..., Name::B => ..., }`, and it can produce a value.
- **Exhaustive:** every variant must be handled, or it's a compile error (E0004, "non-exhaustive patterns").
- **Why:** it turns "forgot a case" bugs into compile errors, and typos can't exist.
- **In loglens:** `LogLevel` replaces the text comparisons.

Add to the compiler-error notebook: *E0004 non-exhaustive patterns*, plus your result from the `;` experiment in `level_of` if you ran it.

**Dashboard update:**

```
Understood (explained or written by me): let/mut, loops, if/else, counters, functions, return values
Introduced: enums, match
Current milestone: loglens v0 -> v1 (enum-based classification)
Compiler errors seen: E0384, linker error (+ add new ones)
```

Send me your finished code and what the compiler said for each experiment.


Your code is correct and the output is right: `Errors: 2`, `Warnings: 2`, `Info: 1`. A few things you did well:

- You built the `enum`, the function that returns it, and the `match` without a template.
- You changed the log text to your own format. That tells me you understood the code instead of copying it.
- `match` arms with `+= 1` work because each arm does an action and produces no value. That's valid, and you used it correctly.

**Two small notes:**

1. You dropped `: u32` from the counters, so Rust picked `i32` for them. It still runs, but a count can never be negative, so `let mut error_count: u32 = 0;` states your intent. Put it back.
2. You didn't mention the two break experiments. Do them in the next 2 minutes, since they're where `match` actually sinks in. Delete one arm, then read what the compiler says is missing. Then try the typo `LogLevel::Erorr`.

## A real bug hiding in your code

Try changing one line of your text to this:

```
INFO: no ERROR found today
```

Run it. It counts as an **error**, because `level_of` only checks whether the word `ERROR` appears *anywhere* in the line. A log line's level is whatever it **starts with**, and the rest is just a message. That's the next fix, and it leads to our next concept.

## Session 5: Structs and methods

### Concept 1: `starts_with`

```rust
line.starts_with("ERROR")   // true only if the line begins with ERROR
```

Swap `contains` for `starts_with` in `level_of` and the bug is gone.

### Concept 2: Struct

An enum says "one of these options". A **struct** says "all of these fields together":

```rust
struct LogEntry {
    level: LogLevel,
    text: String,
}
```

Creating one and reading its fields:

```rust
let entry = LogEntry {
    level: LogLevel::Warn,
    text: String::from("Low memory"),
};

println!("{}", entry.text);   // dot gives access to a field
```

A parsed log line has a level *and* its text, so they belong in one value.

### Concept 3: `String` vs `&str` (your first ownership idea)

You've been using `&str`: text that lives somewhere else and that you only *look at*. `String` is text that your value **owns**, and it lives as long as the value does. Because the struct must keep its text even after the original `line` is gone, the field is a `String`. To convert, use `line.to_string()`.

Think of `&str` as reading someone else's book and `String` as owning your own copy. We'll go deeper on Day 5, and this is the first small taste.

### Concept 4: Methods with `impl`

Functions attached to a struct go in an `impl` block:

```rust
impl LogEntry {
    fn is_error(&self) -> bool {
        match self.level {
            LogLevel::Error => true,
            _ => false,
        }
    }
}
```

- `&self` means "look at the entry this method is called on, without taking it".
- You call it as `entry.is_error()`.
- `_` in a `match` means "anything else". It's the one exception to listing every variant, so use it carefully, because it hides the "forgot a case" protection.

### Your task

```rust
// keep your LogLevel enum and level_of (change contains to starts_with)

struct LogEntry {
    // fields: level (LogLevel) and text (String)
}

fn parse_line(line: &str) -> LogEntry {
    // build and return a LogEntry
    // level: from level_of(line)
    // text: line.to_string()
}

fn main() {
    // same text as now, plus the new tricky line: "INFO: no ERROR found today"

    // keep the three u32 counters

    for line in text.lines() {
        let entry = parse_line(line);

        // match on entry.level and count, as before

        // NEW: if the entry's level is Error, also print:
        //      "ERROR LINE -> <entry.text>"
        // (use a match or an if with your own method)
    }

    // same three summary prints
}
```

Expected: the tricky line must count as **Info**, and only the real error lines get printed with their text.

**Break it on purpose:** try to print an entry whole with `println!("{}", entry);`. Read the error. Rust doesn't know how to display your own struct, and we'll fix that properly soon.

**Stretch:** move your "is it an error?" check into an `impl LogEntry { fn is_error(&self) -> bool }` method and call `entry.is_error()` in `main`.

### WHAT TO WRITE IN MY NOTES

Create `notes/concepts/05-structs-and-impl.md`:

- **`starts_with` vs `contains`:** use `starts_with` when position matters. `contains` caused a real misclassification bug.
- **Struct:** `struct Name { field: Type, }` groups related data, and you read fields with `entry.field`.
- **`String` vs `&str`:** `String` owns its text, `&str` borrows it. Convert with `.to_string()` or `String::from(...)`.
- **`impl`:** methods that belong to a type, taking `&self` to read the value.
- **`_` in `match`:** "anything else". It is convenient but removes the exhaustiveness safety net.
- **Common error:** a struct can't be printed with `{}` unless it implements display (E0277).

**Dashboard:**

```
Introduced: structs, impl, String vs &str, starts_with
Concepts written independently: functions, enum, match (LogLevel)
Bug found by testing: contains vs starts_with misclassification
Next milestone: v1, parse into entries, then read from a file
```

Send me your finished code, the output on the tricky line, and what Rust said for the `{}` experiment.