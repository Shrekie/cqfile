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

    &command.description
  }
}

pub struct Command {
  description: String,
}

impl Command {
  pub fn new(description: String) -> Command {
    Command { description }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_command_and_check_info_exists() {
    let mut invoker = Invoker::new();
    let search = Command::new(String::from("Show all lines with query in it"));
    invoker.enable(String::from("search"), &search);

    assert_eq!(invoker.info("search"), "Show all lines with query in it");
  }
}
