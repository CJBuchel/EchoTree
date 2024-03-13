use super::tree_map::TreeMap;


pub struct TreeHierarchy {
  db: sled::Db,
  hierarchy: sled::Tree,
  metadata_path: String,
}

impl TreeHierarchy {
  pub fn open(db: &sled::Db, metadata_path: String) -> TreeHierarchy {
    let hierarchy = db.open_tree(format!("{}/hierarchy", metadata_path)).expect("open_tree failed");
    TreeHierarchy { db: db.clone(), hierarchy, metadata_path }
  }

  pub fn get_tree_map(&self) -> TreeMap {
    let mut tree_map = TreeMap::new(&self.db, self.metadata_path.clone());

    self.hierarchy.iter().for_each(|tree| {
      let (k, _) = tree.unwrap_or_default();
      let k = std::str::from_utf8(&k).unwrap();
      println!("opening tree: {}", k);

      if !k.starts_with(self.metadata_path.as_str()) {
        tree_map.open_tree(k.to_string());
      } else {
        println!("skipping metadata tree: {}", k);
      }
    });

    tree_map
  }

  // clears all the values in the hierarchy tree (does not delete the tree itself)
  pub fn clear(&self) {
    self.hierarchy.clear().expect("clear failed");
  }

  // drops the hierarchy tree, not recoverable unless a new hierarchy tree is created
  pub fn drop(&self) {
    self.db.drop_tree(self.hierarchy.name()).expect("drop_tree failed");
  }

  pub fn insert_schema(&self, tree: String, schema: String) {
    // update the hierarchy tree with the new tree
    let schema = schema.to_string();
    self.hierarchy.insert(tree, schema.as_bytes()).expect("insert failed");
  }

  pub fn get_schema(&self, tree: String) -> String {
    // get the schema from the hierarchy tree
    let schema = self.hierarchy.get(tree).expect("get failed");
    match schema {
      Some(schema) => std::str::from_utf8(&schema).unwrap().to_string(),
      None => "".to_string()
    }
  }

  pub fn remove_schema(&self, tree: String) {
    self.hierarchy.remove(tree).expect("remove failed");
  }

  pub fn iter(&self) -> sled::Iter {
    self.hierarchy.iter()
  }
}