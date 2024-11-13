use std::io::{self, Write};

// Define the structure for storing text operations
#[derive(Clone)]
enum Operation {
    Insert(usize, char),
    Delete(usize, char),
}

// Linked list node structure
struct HistoryNode {
    operation: Operation,
    next: Option<Box<HistoryNode>>,
}

// Text editor structure
struct TextEditor {
    content: String,
    history: Option<Box<HistoryNode>>,
}

impl TextEditor {
    fn new() -> TextEditor {
        TextEditor {
            content: String::new(),
            history: None,
        }
    }

    // Insert a character at specified position
    fn insert(&mut self, position: usize, ch: char) {
        if position <= self.content.len() {
            // Store the operation in history
            let operation = Operation::Insert(position, ch);
            self.add_to_history(operation);

            // Perform the insertion
            self.content.insert(position, ch);
        }
    }

    // Delete a character at specified position
    fn delete(&mut self, position: usize) {
        if position < self.content.len() {
            let ch = self.content.chars().nth(position).unwrap();

            // Store the operation in history
            let operation = Operation::Delete(position, ch);
            self.add_to_history(operation);

            // Perform the deletion
            self.content.remove(position);
        }
    }

    // Add operation to history
    fn add_to_history(&mut self, operation: Operation) {
        let new_node = Box::new(HistoryNode {
            operation,
            next: self.history.take(),
        });
        self.history = Some(new_node);
    }

    // Undo the last operation
    fn undo(&mut self) -> bool {
        if let Some(node) = self.history.take() {
            match node.operation {
                Operation::Insert(position, _) => {
                    self.content.remove(position);
                }
                Operation::Delete(position, ch) => {
                    self.content.insert(position, ch);
                }
            }
            self.history = node.next;
            true
        } else {
            false
        }
    }

    // Display current content
    fn display(&self) {
        println!("Current text: {}", self.content);
    }
}

fn main() {
    let mut editor = TextEditor::new();
    let mut position = 0;

    println!("Simple Text Editor");
    println!("Commands:");
    println!("i <char> - Insert character at current position");
    println!("d - Delete character at current position");
    println!("m <num> - Move cursor position");
    println!("u - Undo last operation");
    println!("q - Quit");

    loop {
        editor.display();
        println!("Cursor position: {}", position);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let command = input.chars().next().unwrap();
        match command {
            'i' => {
                if let Some(ch) = input.chars().nth(2) {
                    editor.insert(position, ch);
                    position += 1;
                }
            }
            'd' => {
                if position < editor.content.len() {
                    editor.delete(position);
                }
            }
            'm' => {
                if let Some(pos) = input[2..].trim().parse::<usize>().ok() {
                    if pos <= editor.content.len() {
                        position = pos;
                    }
                }
            }
            'u' => {
                if editor.undo() {
                    if position > 0 {
                        position -= 1;
                    }
                } else {
                    println!("Nothing to undo!");
                }
            }
            'q' => break,
            _ => println!("Invalid command!"),
        }
    }
}
