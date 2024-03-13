

pub fn open_db() {
  let db: sled::Db = sled::open("my_db").unwrap();

  db.insert(b"test", b"test").expect("insert failed");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    open_db();
  }
}
