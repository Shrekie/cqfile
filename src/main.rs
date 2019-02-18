use cqfile::terminal::scanner;
use std::env;
use cqfile::terminal::expression::Count;
use cqfile::terminal::expression::Search;
use terminal::expression::Command;
use cqfile::terminal::Invoker;

fn main() {
  let input: Vec<String> = env::args().collect();

  let query = scanner::encode(&input);
  let file_contents = scanner::read(&query.filename);
  let mut invoker = Invoker::new();

  let search = Search::new();
  invoker.enable("search", &search);
  let counts = Count::new();
  invoker.enable("count", &counts);

  let command = invoker.get(&query.command);
  let result = command.operate(&query.argument, &file_contents);

  match result {
    Ok(v) => v.show(),
    Err(_e) => println!("ERROREL"),
  };
}
