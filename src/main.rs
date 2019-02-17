use cqfile::cmd_query;
use cqfile::terminal::expression::CommandResult;
use cqfile::terminal::scanner;
use std::env;

fn main() {
  let input: Vec<String> = env::args().collect();
  let query = scanner::encode(&input);
  let contents = scanner::read(&query.filename);

  match cmd_query(&query, &contents) {
    CommandResult::Number(n) => println!("{}", n),
    CommandResult::Lines(l) => {
      for line in l {
        println!("{}", line);
      }
    }
  };
}
