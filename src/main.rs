// Session 2: Structs, impl, Self, ::new() constructors
//
// Running example: the `Task` type of a small todo CLI.
//
// Exercise 1: `cargo run` -> read the compiler error, then fix it.
//             Rule: NO `.clone()`. Only change the receiver of `label`.
// Exercise 2: implement the constructors `Task::new` and `Task::new_done`.
//             `new_done` should reuse `Self::new`.
// Exercise 3: implement the methods `is_done`, `complete` and `title`.
//             Their signatures are given. Before you start, say why each one
//             has the receiver it has.
// Exercise 4: implement `into_title` and `renamed`. Both take `self`.
//             `renamed` must use struct update syntax (`..self`).
// Exercise 5: implement `TaskId::first` and `TaskId::next` (tuple struct).
// Check:      `cargo test` -> all green.

fn main() {
    let mut task = Task::new("Write tests");
    println!("{}", task.label());
    task.complete();
    println!("{}", task.label());
}

struct Task {
    title: String,
    done: bool,
}

impl Task {
    /// Creates an open task with the given title.
    fn new(title: &str) -> Self {
        todo!()
    }

    /// Creates a task that is already done. Reuse `Self::new`.
    fn new_done(title: &str) -> Self {
        todo!()
    }

    /// Returns whether the task is done.
    fn is_done(&self) -> bool {
        todo!()
    }

    /// Marks the task as done.
    fn complete(&mut self) {
        todo!()
    }

    /// Returns the title without copying it.
    fn title(&self) -> &str {
        todo!()
    }

    /// One line for the CLI: "[ ] Write tests" or "[x] Write tests".
    fn label(self) -> String {
        let mark = if self.done { 'x' } else { ' ' };
        format!("[{mark}] {}", self.title)
    }

    /// Gives away the owned title. Afterwards the task is gone.
    fn into_title(self) -> String {
        todo!()
    }

    /// Returns the same task with a new title; `done` stays as it was.
    /// Use struct update syntax: `Self { title: …, ..self }`.
    fn renamed(self, title: &str) -> Self {
        todo!()
    }
}

/// Newtype for task ids: a `TaskId` can't be mixed up with any other `u32`.
#[derive(Debug, PartialEq)] // derive = session 9; needed so assert_eq! can compare and print ids
struct TaskId(u32);

impl TaskId {
    /// The first id is 1.
    fn first() -> Self {
        todo!()
    }

    /// The id after this one. `self` stays usable.
    fn next(&self) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_open() {
        let t = Task::new("Write tests");
        assert_eq!(t.title(), "Write tests");
        assert!(!t.is_done());
    }

    #[test]
    fn new_done_is_done() {
        let t = Task::new_done("Ship it");
        assert_eq!(t.title(), "Ship it");
        assert!(t.is_done());
    }

    #[test]
    fn complete_marks_done() {
        let mut t = Task::new("Write tests");
        t.complete();
        assert!(t.is_done());
        t.complete(); // completing twice is fine
        assert!(t.is_done());
    }

    #[test]
    fn label_does_not_consume() {
        let mut t = Task::new("Write tests");
        assert_eq!(t.label(), "[ ] Write tests");
        t.complete();
        assert_eq!(t.label(), "[x] Write tests");
    }

    #[test]
    fn into_title_gives_owned_string() {
        let t = Task::new("Write tests");
        let mut title: String = t.into_title();
        title.push('!');
        assert_eq!(title, "Write tests!");
    }

    #[test]
    fn renamed_keeps_done() {
        let t = Task::new_done("Old").renamed("New");
        assert_eq!(t.title(), "New");
        assert!(t.is_done());

        let t = Task::new("Old").renamed("New");
        assert!(!t.is_done());
    }

    #[test]
    fn task_ids_count_up() {
        let a = TaskId::first();
        let b = a.next();
        let c = b.next();
        assert_eq!(a, TaskId(1));
        assert_eq!(b, TaskId(2));
        assert_eq!(c.0, 3);
    }
}
