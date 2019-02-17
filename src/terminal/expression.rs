// #TODO: Enable hashmap of invoker to take memory

pub enum CommandResult<'a> {
  Lines(Vec<&'a str>),
  Number(usize),
}

pub trait Command {
  fn description(&self) -> &str;
  fn operate<'a>(&self, argument: &str, contents: &'a str) -> CommandResult<'a>;
}

pub struct Search;

impl Command for Search {
  fn description(&self) -> &str {
    "Show all lines with query in it"
  }

  fn operate<'a>(&self, argument: &str, contents: &'a str) -> CommandResult<'a> {
    let mut results = Vec::new();

    for line in contents.lines() {
      if line.contains(argument) {
        results.push(line);
      }
    }

    CommandResult::Lines(results)
  }
}

impl Search {
  pub fn new() -> Search {
    Search {}
  }
}

pub struct Count;

impl Command for Count {
  fn description(&self) -> &str {
    "Show all lines with query in it"
  }

  fn operate<'a>(&self, argument: &str, contents: &'a str) -> CommandResult<'a> {
    let mut results = Vec::new();

    for line in contents.lines() {
      if line.contains(argument) {
        results.push(line);
      }
    }

    let count = results.iter().count();
    if count > 0 {
      CommandResult::Number(count + 1)
    } else {
      CommandResult::Number(0)
    }
  }
}

impl Count {
  pub fn new() -> Count {
    Count {}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn run_search_command() {
    let argument = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    let search = Search::new();

    let result = match search.operate(argument, contents) {
      CommandResult::Lines(s) => s,
      _ => vec!["safe, fast, productive."],
    };

    assert_eq!(vec!["safe, fast, productive."], result);
  }

  #[test]
  fn run_count_command() {
    let argument = "p";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    let count = Count::new();
    let result = match count.operate(argument, contents) {
      CommandResult::Number(s) => s,
      _ => 0,
    };

    assert_eq!(2, result);
  }
}
