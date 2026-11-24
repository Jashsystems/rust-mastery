Fair enough. The counting idea is the part that isn't clicking, so here's the full code. After it, I'll walk through what happens step by step.

```rust
fn main() {
    let text = "INFO start\nERROR disk full\nWARN slow response\nINFO user login\nERROR timeout\nINFO done";

    let mut errors = 0;
    let mut warnings = 0;
    let mut infos = 0;

    for line in text.lines() {
        if line.contains("ERROR") {
            errors += 1;
        } else if line.contains("WARN") {
            warnings += 1;
        } else {
            infos += 1;
        }
    }

    println!("errors: {}", errors);
    println!("warnings: {}", warnings);
    println!("infos: {}", infos);
}
```

## How the counting works

Think of the three variables as **tally counters**, like the clicker a person at a door uses to count visitors. Each counter starts at 0. Every time you see a matching line, you click once (`+= 1`).

The loop looks at one line at a time, and the counters remember their totals between lines. Here is the whole run:

| Round | `line` | Which branch matches | errors | warnings | infos |
|---|---|---|---|---|---|
| start | | | 0 | 0 | 0 |
| 1 | `INFO start` | else (no ERROR, no WARN) | 0 | 0 | 1 |
| 2 | `ERROR disk full` | first `if` | 1 | 0 | 1 |
| 3 | `WARN slow response` | `else if` | 1 | 1 | 1 |
| 4 | `INFO user login` | else | 1 | 1 | 2 |
| 5 | `ERROR timeout` | first `if` | 2 | 1 | 2 |
| 6 | `INFO done` | else | 2 | 1 | 3 |

After the loop ends, the three `println!` lines show the final totals: `errors: 2`, `warnings: 1`, `infos: 3`.

## Why the counters sit above the loop

If you put `let mut errors = 0;` inside the loop, a brand new counter would be created on every round and thrown away at the end of it. The count would reset to 0 each time, and you'd lose your total. Creating the counters before the loop makes them live for the whole loop. This is the *scope* idea from last time. Once you try it, Rust will also complain that `errors` doesn't exist when you go to print it after the loop.

## Your turn (smaller, and now you have a model)

Copy the code above into `src\main.rs`, run `cargo run`, and check that the output matches. Then make **one** change: add a new level called `DEBUG`, with its own counter.

1. Add a line `DEBUG cache hit` to the text, and add one more line starting with `DEBUG`.
2. Add `let mut debugs = 0;` with the other counters.
3. Add a branch that counts lines containing `"DEBUG"`.
4. Print `debugs` too.

Think about where the new `else if` should go, and what happens to the `else` at the bottom. Expected output with two DEBUG lines added:

```
errors: 2
warnings: 1
infos: 3
debugs: 2
```

If that works, you've written the core of loglens yourself, and we'll move on to storing this data properly.

## WHAT TO WRITE IN MY NOTES

Add to `notes/concepts/02-text-loops-if.md`:

- **Counter pattern:** `let mut n = 0;` before the loop, `n += 1;` inside when something matches, print after the loop.
- **Why before the loop:** variables made inside `{ }` disappear when the `{ }` ends, so a counter made inside the loop would reset every round.
- **`else if` order:** the first matching branch wins and the rest are skipped.

Paste your DEBUG version and tell me what you decided about the `else` at the bottom.