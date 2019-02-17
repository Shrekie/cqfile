// #TODO: Enable hashmap of invoker to take memory

pub enum CommandResult<'a> {
  Lines(Vec<&'a str>),
  Sum(usize),
}

pub trait Command {
  fn description(&self) -> &str;
  fn operate<'a>(&self, argument: &str, contents: &'a str) -> Result<CommandResult<'a>, &'a str>;
}

pub struct Search;

impl Command for Search {
  fn description(&self) -> &str {
    "Show all lines with query in it"
  }

  fn operate<'a>(&self, argument: &str, contents: &'a str) -> Result<CommandResult<'a>, &'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
      if line.contains(argument) {
        results.push(line);
      }
    }
  
    Ok(CommandResult::Lines(results))
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
    "Count all times query matches"
  }

  fn operate<'a>(&self, argument: &str, contents: &'a str) -> Result<CommandResult<'a>, &'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
      if line.contains(argument) {
        results.push(line);
      }
    }

    let mut count = results.iter().count();
    if count > 0 {
      count = count + 1
    };

    Ok(CommandResult::Sum(count))
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

    match search.operate(argument, contents) {
      Ok(CommandResult::Lines(l)) => assert_eq!(vec!["safe, fast, productive."], l),
      Err(e) => println!("{}", e),
      _ => (),
    };
  }

  #[test]
  fn run_count_command() {
    let argument = "p";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    let count = Count::new();

    match count.operate(argument, contents) {
      Ok(CommandResult::Sum(s)) => assert_eq!(2, s),
      Err(e) => println!("{}", e),
      _ => (),
    };
  }
}
