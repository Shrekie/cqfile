pub mod terminal;
use terminal::expression::Command;
use terminal::expression::CommandResult;
use terminal::expression::Count;
use terminal::expression::Search;
use terminal::expression::Line;
use terminal::scanner;
use terminal::Invoker;
use terminal::Query;

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

    match command.operate(&query.argument, &file_contents).and_then(|l| l.display()).and_then(|r| r.as_value()) {
      Ok(r) => assert_eq!(vec!["safe, fast, productive."], *r),
      Err(e) => println!("{}", e),
      _ => (),
    };
  }
}
