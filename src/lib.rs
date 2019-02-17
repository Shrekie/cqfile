pub mod terminal;

use terminal::expression::CommandResult;
use terminal::expression::Count;
use terminal::expression::Search;
use terminal::scanner;
use terminal::Invoker;
use terminal::Query;

pub fn cmd_query<'a>(query: &Query, file_contents: &'a str) -> Result<CommandResult<'a>, &'a str> {
  let mut invoker = Invoker::new();

  let search = Search::new();
  invoker.enable("search", &search);
  let counts = Count::new();
  invoker.enable("count", &counts);

  let command = invoker.get(&query.command);
  let result = command.operate(&query.argument, file_contents);

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

    match command.operate(&query.argument, &file_contents) {
      Ok(CommandResult::Lines(l)) => assert_eq!(vec!["hello there"], l),
      Err(e) => println!("{}", e),
      _ => (),
    };
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

    match cmd_query(&query, &contents) {
      Ok(CommandResult::Lines(l)) => assert_eq!(vec!["hello there"], l),
      Err(e) => println!("{}", e),
      _ => (),
    };
  }

}
