use sled::IVec;


pub struct ManagedTree {
  db: sled::Db,
  tree_name: String,
  tree: sled::Tree,
  checksum: u32,
}

impl ManagedTree {
  pub fn new(db: &sled::Db, tree_name: String) -> Result<Self, sled::Error> {
    let tree = match db.open_tree(tree_name.clone()) {
      Ok(t) => t,
      Err(e) => {
        log::error!("open_tree failed for {}: {}", tree_name, e);
        return Err(e);
      }
    };


    let checksum = match tree.checksum() {
      Ok(c) => c,
      Err(e) => {
        log::error!("checksum failed for {}: {}", tree_name, e);
        0
      }
    };

    Ok(ManagedTree { db: db.clone(), tree_name, tree, checksum })
  }

  fn checksum(&mut self) {
    match self.tree.checksum() {
      Ok(c) => self.checksum = {
        log::debug!("{}: checksum: {}", self.tree_name, c);
        c
      },
      Err(e) => log::error!("checksum failed for {}: {}", self.tree_name, e)
    }
  }

  pub fn drop(&mut self) -> Result<bool, sled::Error> {
    self.db.drop_tree(self.tree_name.as_str())
  }

  pub fn get_checksum(&self) -> u32 {
    self.checksum
  }

  pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<IVec>, sled::Error> {
    let result = self.tree.insert(key, value);

    // checksum after each time
    if result.is_ok() {
      self.checksum();
    }

    result
  }

  pub fn get(&self, key: &[u8]) -> Result<Option<IVec>, sled::Error> {
    self.tree.get(key)
  }

  pub fn remove(&mut self, key: &[u8]) -> Result<Option<IVec>, sled::Error> {
    let result = self.tree.remove(key);

    // checksum after each time
    if result.is_ok() {
      self.checksum();
    }

    result
  }

  pub fn clear(&mut self) -> Result<(), sled::Error> {
    let result = self.tree.clear();

    // checksum after each time
    if result.is_ok() {
      self.checksum();
    }

    result
  }

  pub fn iter(&self) -> sled::Iter {
    self.tree.iter()
  }

  pub fn name(&self) -> String {
    self.tree_name.clone()
  }
}