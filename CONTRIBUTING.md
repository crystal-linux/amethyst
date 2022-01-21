# Crystal Linux Contributing Guidelines

#### !! Always make sure to `git pull` before doing any work to avoid commit hell !!

### Pre-Commit Checks

- Make sure to `cargo fmt` your code before every commit push
- Unless in specific edge cases, don't push code that doesn't pass `cargo check`
- Try to correct any code with `cargo clippy` before you push

### Formatting

- UNIX line endings (LF instead of CRLF)
- 4 spaces per TAB

### Good Practices

- Try to use .unwrap() as little as possible
- Try to never use panic!() in production code, always try to have a possible way to resolve errors, even if it's just
  unwrap_or/_else()
- Never use println!() or eprintln!() in finalised code. Using string functions (e.g. info() in Amethyst v3.0.0) is
  preferred
- Compartmentalise as much as you can, avoid writing the exact same line of code 50 times if you can turn it into a
  function

### Examples of these guidelines in practice

- https://git.getcryst.al/crystal/ame/src/branch/rewrite