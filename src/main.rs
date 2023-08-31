mod parser;
pub mod error;
pub mod types;

use parser::Parser;

fn main() {
  let query = "SELECT * FROM users WHERE userId=1;";
  let insert_query = "INSERT INTO user(userId, fname, lname, email, password) VALUES(1, 'Sankar', 'Boro', 'sankar.boro@yahoo.com', 'sankar')";
  let data = Parser::new(&insert_query).parse();
  println!("{:?}", data);
}