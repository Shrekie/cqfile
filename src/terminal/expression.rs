// #TODO: Enable hashmap of invoker to take memory
pub trait Command {
  fn description(&self) -> &str;
  fn operate<'a>(&self, argument: &str, contents: &'a str) -> Vec<&'a str>;
}

pub struct Search;

impl Command for Search {
  fn description(&self) -> &str {
    "Show all lines with query in it"
  }

  fn operate<'a>(&self, argument: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
      if line.contains(argument) {
        results.push(line);
      }
    }

    results
  }
}

impl Search {
  pub fn new() -> Search {
    Search {}
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
    assert_eq!(
      vec!["safe, fast, productive."],
      search.operate(argument, contents)
    );
  }
}