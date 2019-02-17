pub mod terminal;

use terminal::expression::CommandResult;
use terminal::expression::Search;
use terminal::expression::Count;
use terminal::scanner;
use terminal::Invoker;
use terminal::Query;

pub fn cmd_query<'a>(query: &Query, file_contents: &'a str) -> CommandResult<'a> {

  let mut invoker = Invoker::new();

  let search = Search::new();
  invoker.enable("search", &search);
  let count = Count::new();
  invoker.enable("count", &count);

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

    let result = match command.operate(&query.argument, &file_contents) {
      CommandResult::Lines(s) => s,
      _ => vec!["safe, fast, productive."],
    };

    assert_eq!(vec!["hello there"], result);
  }

  #[test]
  fn run_command() {
    let input = vec![
      String::from("cqfile"),
      String::from("file.txt"),
      String::from("search"),
      String::from("there"),
    ];
    let query = scanner::encode(&input);
    let contents = scanner::read(&query.filename);
    let result = match cmd_query(&query, &contents) {
      CommandResult::Lines(s) => s,
      _ => vec!["MAKE ME BOXED WITH RESULT"],
    };

    assert_eq!(vec!["hello there"], result);
  }

}
