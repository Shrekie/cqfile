pub mod terminal;

use terminal::expression::Search;
use terminal::scanner;
use terminal::Invoker;
use terminal::Query;

pub fn cmd_query<'a>(query: &Query, file_contents: &'a str) -> Vec<&'a str> {
  let mut invoker = Invoker::new();
  let search = Search::new();
  invoker.enable("search", &search);

  let command = invoker.get(&query.command);
  let result = command.operate(&query.argument, &file_contents);

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn run_search_command_with_invoked_query() {
    let input = [
      String::from("cqfile"),
      String::from("file.txt"),
      String::from("search"),
      String::from("there"),
    ];

    let mut invoker = Invoker::new();
    let search = Search::new();
    invoker.enable("search", &search);

    let command = invoker.get(&input[2]);
    let query = scanner::encode(&input);
    let file_contents = scanner::read(&query.filename);
    let result = command.operate(&query.argument, &file_contents);

    assert_eq!(vec!["hello there"], result);
  }

  #[test]
  fn run_command() {
    let input = vec!(
      String::from("cqfile"),
      String::from("file.txt"),
      String::from("search"),
      String::from("there"),
    );
    let query = scanner::encode(&input);
    let contents = scanner::read(&query.filename);
    assert_eq!(vec!["hello there"], cmd_query(&query, &contents));
  }

}
