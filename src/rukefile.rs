use std::{fs, io};

pub fn get_rukefile(path: &str) -> Result<String, io::Error> {
  match path {
    "" => fs::read_to_string("Rukefile"),
    _ => fs::read_to_string(path)
  }
}

pub fn execute_task(name: &str) {
  println!("{}", name)
}
