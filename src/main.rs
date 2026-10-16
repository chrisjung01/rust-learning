// Session 1: Ownership & Borrowing
//
// Exercise 1: `cargo run` -> read the compiler error, then fix it.
//             Rule: NO `.clone()`. Only change the signature of `make_greeting`.
// Exercise 2: implement `append_bang` (replace todo!).
// Exercise 3: implement `first_word`.
// Check:      `cargo test` -> all green.

fn main() {
    let name = String::from("Flutter");
    let greeting = make_greeting(&name);
    println!("{greeting}");
    println!("{name} is still here");
}

fn make_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Appends "!" to the string without creating a new one.
fn append_bang(s: &mut String) {
    s.push('!');
}

/// Returns the first word (up to the first space) as a slice, without copying.
/// No space -> the whole text.
fn first_word(text: &str) -> &str {
    text.split(" ").next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting() {
        assert_eq!(make_greeting("Dart"), "Hello, Dart!");
    }

    #[test]
    fn bang_twice() {
        let mut s = String::from("Hi");
        append_bang(&mut s);
        append_bang(&mut s);
        assert_eq!(s, "Hi!!");
    }

    #[test]
    fn first_word_works() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
        assert_eq!(first_word(""), "");
    }
}
