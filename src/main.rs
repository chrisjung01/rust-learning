// Session 1: Ownership & Borrowing
//
// Aufgabe 1: `cargo run` -> Compiler-Fehler lesen, dann fixen.
//            Regel: KEIN `.clone()`. Nur die Signatur von `make_greeting` ändern.
// Aufgabe 2: `append_bang` implementieren (todo! ersetzen).
// Aufgabe 3: `first_word` implementieren.
// Check:     `cargo test` -> alles grün.

fn main() {
    let name = String::from("Flutter");
    let greeting = make_greeting(&name);
    println!("{greeting}");
    println!("{name} ist immer noch da");
}

fn make_greeting(name: &str) -> String {
    format!("Hallo, {name}!")
}

/// Hängt "!" an den String an, ohne einen neuen zu erzeugen.
fn append_bang(s: &mut String) {
    s.push('!');
}

/// Gibt das erste Wort zurück (bis zum ersten Leerzeichen) – als Slice, ohne Kopie.
/// Kein Leerzeichen -> ganzer Text.
fn first_word(text: &str) -> &str {
    text.split(" ").next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting() {
        assert_eq!(make_greeting("Dart"), "Hallo, Dart!");
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
