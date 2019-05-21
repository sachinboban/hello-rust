# Formatted Print
The following macros are provided in `std::fmt`:
* `format!`: Convert to string (similar to `sprintf()`),
* `print!`: Similar to `printf()`, writes to console `io::stdout`
* `println`: `print!` with newline appended.
* `eprint!`: Same as `print!`, but writes to standard error `io::stderr`
* `eprintln`: Same as `eprint!`, but with newline appended.

## Arguments to print macros
`{}` will be automatically replaced with any arguments.
* `{0}`: First argument
* `{1}`: Second argument
* etc

## Named arguments
Arguments can be passed as named.
```
println!("{subject} {verb} {object}.",
         object="the lazy dog",
         subject="The quick brown fox",
         verb="jumps over");
```

## Reference
https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
