use std::collections::HashMap;
use std::error::Error;

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

  pub fn info(&self, name: &'a str) -> &'a str {
    match self.commands.get(name) {
      Some(_x) => name,
      None => "noo",
    }
  }
}

pub struct Command {
  //fn new(&self) -> Result<(), Box<dyn Error>>;
}

pub struct Search {
  argument: u32,
}

impl Search {
  pub fn new() -> Command {
    Command {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn add_command_and_check_info_exists() {
    let mut invoker = Invoker::new();
    let search = Search::new();
    invoker.enable("search", &search);

    assert_eq!(invoker.info("search"), "search");
  }
}
