use std::collections::HashMap;
use std::process;

pub struct Invoker<'a> {
  pub commands: HashMap<String, &'a Command>,
}

impl<'a> Invoker<'a> {
  pub fn new() -> Invoker<'a> {
    return Invoker {
      commands: HashMap::new(),
    };
  }

  pub fn enable(&mut self, name: String, command: &'a Command) {
    self.commands.insert(name, command);
  }

  pub fn info(&self, name: &'a str) -> &String {
    let command = self.commands.get(name).unwrap_or_else(|| {
      println!("Could not find command");
      process::exit(1);
    });

    command.description()
  }
}

pub struct Query {
  pub target: String,
  pub filename: String,
  pub argument: String,
}

impl Query {
  fn new(target: String, filename: String, argument: String) -> Query {
    Query {
      target,
      filename,
      argument,
    }
  }

  fn parts(&self) -> (&String, &String, &String) {
    (&self.target, &self.filename, &self.argument)
  }
}

mod scanner {
  use super::*;

  pub fn encode(args: &[String]) -> Query {
    if args.len() < 4 {
      println!("Too few arguments");
      process::exit(1);
    }

    let target = args[1].clone();
    let command = args[2].clone();
    let argument = args[3].clone();

    Query::new(target, command, argument)
  }
}

pub trait Command {
  fn description(&self) -> &String;
}

pub struct Search {
  pub description: String,
}

impl Command for Search {
  fn description(&self) -> &String {
    &self.description
  }
}

impl Search {
  fn new(description: String) -> Search {
    Search { description }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_command_and_check_info_exists() {
    let mut invoker = Invoker::new();
    let search = Search::new(String::from("Show all lines with query in it"));
    invoker.enable(String::from("search"), &search);
    assert_eq!(invoker.info("search"), "Show all lines with query in it");
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
}
