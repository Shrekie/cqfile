// #TODO: Enable hashmap of invoker to take memory

pub trait CommandResult {
  type Item;
  type Value;
  fn display(&self) -> Result<&Self::Item, &str>;
  fn as_value(&mut self) -> Result<&Self::Value, &str>;
}

pub struct Line<'a> {
  result: Vec<&'a str>,
}

pub struct Sum {
  result: usize,
}

impl<'a> CommandResult for Line<'a> {
  type Item = Line<'a>;
  type Value = Vec<&'a str>;

  fn display(&self) -> Result<&Self::Item, &str> {
    for line in &self.result {
      println!("{}", line);
    }
    Ok(self)
  }

  fn as_value(&mut self) -> Result<&Self::Value, &str> {
    Ok(&self.result)
  }
}

impl CommandResult for Sum {
  type Item = Sum;
  type Value = usize;

  fn display(&self) -> Result<&Self::Item, &str> {
    println!("{}", self.result);
    Ok(self)
  }

  fn as_value(&mut self) -> Result<&Self::Value, &str> {
    Ok(&self.result)
  }
}

impl Sum {
  pub fn new(result: usize) -> Sum {
    Sum { result }
  }
}

impl<'a> Line<'a> {
  pub fn new(result: Vec<&'a str>) -> Line<'a> {
    Line { result }
  }
}

pub trait Command<'a> {
  type Item;
  fn description(&self) -> &str;
  fn operate(&self, argument: &str, contents: &'a str) -> Result<Self::Item, &'a str>;
}

pub struct Search;

impl<'a> Command<'a> for Search {
  type Item = Line<'a>;

  fn description(&self) -> &str {
    "Show all lines with query in it"
  }

  fn operate(&self, argument: &str, contents: &'a str) -> Result<Self::Item, &'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
      if line.contains(argument) {
        results.push(line);
      }
    }

    Ok(Line::new(results))
  }
}

impl Search {
  pub fn new() -> Search {
    Search {}
  }
}

pub struct Count;

impl<'a> Command<'a> for Count {
  type Item = Sum;

  fn description(&self) -> &str {
    "Count all times query matches"
  }

  fn operate(&self, argument: &str, contents: &'a str) -> Result<Self::Item, &'a str> {
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

    Ok(Sum::new(count))
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

    match search
      .operate(argument, contents)
      .and_then(|l| l.display())
      .and_then(|r| r.as_value())
    {
      Ok(r) => assert_eq!(vec!["safe, fast, productive."], *r),
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

    match count.operate(argument, contents).and_then(|s| s.display()) {
      Ok(s) => assert_eq!(2, *s),
      Err(e) => println!("{}", e),
      _ => (),
    };
  }
}
