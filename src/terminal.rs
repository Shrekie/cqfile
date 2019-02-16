use expression::Command;
use std::collections::HashMap;
use std::fs;
use std::process;

pub mod expression;

pub struct Invoker<'a> {
  pub commands: HashMap<&'a str, &'a Command>,
}

impl<'a> Invoker<'a> {
  pub fn new() -> Invoker<'a> {
    return Invoker {
      commands: HashMap::new(),
    };
  }

  pub fn enable(&mut self, name: &'a str, command: &'a Command) {
    self.commands.insert(name, command);
  }

  pub fn get(&self, name: &str) -> &Command {
    let command = self.commands.get(name).unwrap_or_else(|| {
      println!("Could not find command");
      process::exit(1);
    });

    *command
  }
}

pub struct Query {
  pub filename: String,
  pub command: String,
  pub argument: String,
}

impl Query {
  fn new(filename: String, command: String, argument: String) -> Query {
    Query {
      filename,
      command,
      argument,
    }
  }

  fn parts(&self) -> (&String, &String, &String) {
    (&self.filename, &self.command, &self.argument)
  }
}

pub mod scanner {
  use super::*;

  pub fn encode(args: &[String]) -> Query {
    if args.len() < 4 {
      println!("Too few arguments");
      process::exit(1);
    }

    let filename = args[1].clone();
    let command = args[2].clone();
    let argument = args[3].clone();

    Query::new(filename, command, argument)
  }

  pub fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
      Ok(v) => v,
      Err(_e) => {
        println!("Can't read file {}", filename);
        process::exit(1);
      }
    }

  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_command_and_get_command_info() {
    let mut invoker = Invoker::new();
    let search = expression::Search::new();
    invoker.enable("search", &search);
    
    let command = invoker.get("search");
    assert_eq!(command.description(), "Show all lines with query in it");
  }

  #[test]
  fn split_string_input_to_three_query_parts() {
    let input = [
      String::from("cqfile"),
      String::from("file.txt"),
      String::from("search"),
      String::from("Victoria"),
    ];

    let query = scanner::encode(&input);
    let parts = query.parts();
    assert_eq!(
      parts,
      (
        &String::from("file.txt"),
        &String::from("search"),
        &String::from("Victoria"),
      )
    );
  }

  #[test]
  fn read_file_contents() {
    let contents = "\
hello there
not here
hello here";
    assert_eq!(scanner::read("file.txt"), contents);
  }
}
