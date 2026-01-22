/// REPL (Read-Eval-Print Loop) for FORM
use crate::evaluator::Evaluator;
use crate::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub struct Repl {
    evaluator: Evaluator,
    editor: DefaultEditor,
}

impl Repl {
    pub fn new() -> Result<Self, String> {
        let editor = DefaultEditor::new().map_err(|e| format!("Failed to create editor: {}", e))?;
        Ok(Repl {
            evaluator: Evaluator::new(),
            editor,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        println!("FORM REPL v0.1.0");
        println!("A symbolic manipulation system");
        println!("Type 'quit' or 'exit' to exit, 'help' for help\n");

        loop {
            let readline = self.editor.readline("FORM> ");
            match readline {
                Ok(line) => {
                    let line = line.trim();

                    // Skip empty lines
                    if line.is_empty() {
                        continue;
                    }

                    // Add to history
                    let _ = self.editor.add_history_entry(line);

                    // Check for special commands
                    match line {
                        "quit" | "exit" => {
                            println!("Goodbye!");
                            break;
                        }
                        "help" => {
                            self.print_help();
                            continue;
                        }
                        "clear" => {
                            self.evaluator = Evaluator::new();
                            println!("Environment cleared");
                            continue;
                        }
                        _ => {}
                    }

                    // Parse and evaluate
                    match self.evaluate(line) {
                        Ok(result) => {
                            if !result.is_empty() {
                                println!("  {}", result);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("Goodbye!");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    fn evaluate(&mut self, input: &str) -> Result<String, String> {
        let mut parser = Parser::new(input);
        let statement = parser.parse_statement()?;
        self.evaluator.eval_statement(statement)
    }

    fn print_help(&self) {
        println!("\nFORM REPL Help");
        println!("==============\n");
        println!("Commands:");
        println!("  quit, exit       - Exit the REPL");
        println!("  help             - Show this help message");
        println!("  clear            - Clear all definitions\n");
        println!("Syntax:");
        println!("  Symbols x, y, z  - Declare symbols");
        println!("  Expression e = (x + y)^2  - Define an expression");
        println!("  Local a = x + 1  - Define a local variable");
        println!("  id x = 1         - Add substitution rule");
        println!("  Print e          - Print an expression");
        println!("  .sort            - Apply all rules and simplify\n");
        println!("Examples:");
        println!("  > Symbols x, y");
        println!("  > Expression e = (x + 1) * (x - 1)");
        println!("  > id x = 2");
        println!("  > .sort");
        println!("  > Print e");
        println!("  > 2 + 3 * 4");
        println!("  > (1 + 2) ^ 3\n");
    }
}
