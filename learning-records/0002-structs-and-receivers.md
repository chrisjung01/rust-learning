# Structs, constructors and receivers

Solved session 2: chose receivers correctly (`&self` for `label`, `self` for `into_title`/`renamed`), used struct update syntax unprompted in `new_done`, and wrote a tuple-struct newtype. Quiz 8/8.

**Evidence**: tag `session-02-solution`, 7/7 tests green.

**Corrected along the way**:
- Wrote `self.title()` inside the getter `title` (infinite recursion). Field vs. method by parentheses is now explicit.
- Tried `self.0++`: Rust has no `++`, and `next(&self)` must compute a new value, not mutate. Watch for Dart/C habits of in-place mutation where Rust wants a new value.

**Implications**: ready for enums and `match` (session 3). Deref coercion (`&String` → `&str`) was only mentioned, not practised; revisit in session 13.
