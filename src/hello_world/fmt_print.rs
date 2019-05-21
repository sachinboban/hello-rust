/*
 * Formatted Print
 * https://doc.rust-lang.org/stable/rust-by-example/hello/print.html
 */
fn main() {
    println!("{} days", 31);

    /*
     * {0}: Alice
     * {1}: Bob
     */
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    /*
     * Arguments can be named objects as well
     */
    println!("{subject} {verb} {object}.",
             object="the lazy dog",
             subject="The quick brown fox",
             verb="jumps over");
}
