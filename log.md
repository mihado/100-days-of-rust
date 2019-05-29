# 100 Days Of Code - Log

### Day 0: May 28th, 2019

**Today's Progress**: wrote hello_world, and guessing_game. learned about cargo

**Thoughts:** Rust is fun! I **love** pattern matching! At first glance, reference is easier to understand than C pointer. Cargo would definitely helps with dependencies hell. I'm used to the awesomeness of ruby bundler so I cannot use any language without something similar.

**Link to work:** [Hello World!](./00_hello_world), [Guessing Game](./01_guessing_game)

---

### Day 1: May 29th, 2019

**Today's Progress**: learned about variables, scalar and compound data types, functions & expressions, control flow, loops.

**Thoughts:**

* Variable is immutable by default. It can be changed to immutable at initialisation `let mut x`;
* Variable type can be changed by shadowing `let x = "string"; let x: i32 = 5`;
* `CONSTANT` are immutable and cannot be assigned with a computed expression at run time.
* Statement does not return value, and must end with a semi colon.
* Expression must **not** end with a semi colon. The last expression is returned implicitly.
* `tuple ()` and `array []` have fixed length. A tuple can hold multiple types. An array can hold one type.
* All branches in the conditional assignment must have the same type.
* There are 3 types of loop `loop`, `while`, and `for`. A loop can be returned early using `break`. An optional expression can be provided to provide the return value, e.g: `break x + 1`.
