use cqfile::cmd_query;
use cqfile::terminal::scanner;
use std::env;

fn main() {
  let input: Vec<String> = env::args().collect();
  let query = scanner::encode(&input);
  let contents = scanner::read(&query.filename);
  let results = cmd_query(&query, &contents);
  
  for line in results {
    println!("{}", line);
  }
}
