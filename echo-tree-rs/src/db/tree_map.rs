use std::collections::HashMap;


pub struct TreeMap {
  db: sled::Db,
  tree_map: HashMap<String, sled::Tree>,
  metadata_path: String,
}

impl TreeMap {
  pub fn new(db: &sled::Db, metadata_path: String) -> TreeMap {
    TreeMap {
      db: db.clone(),
      tree_map: HashMap::new(),
      metadata_path,
    }
  }

  // clears all the values in every tree (does not delete the trees themselves)
  pub fn clear(&mut self) {
    // clear the trees in the map
    self.tree_map.iter().for_each(|(_, v)| {
      let _ = v.clear();
    });
  }

  // drops all the trees, not recoverable unless new trees are created
  pub fn drop(&mut self) {
    // drop the trees in the map
    self.tree_map.iter().for_each(|(k, _)| {
      let _ = self.db.drop_tree(k);
    });

    self.tree_map.clear();
  }


  pub fn open_tree(&mut self, tree: String) {
    if self.tree_map.contains_key(&tree) {
      println!("tree already exists: {}", tree);
      return
    }

    if tree.starts_with(self.metadata_path.as_str()) {
      println!("metadata trees are forbidden: {}", tree);
      return
    }

    let t = self.db.open_tree(tree.clone()).expect(format!("open_tree failed for {}", tree).as_str());
    self.tree_map.insert(tree, t);
  }

  pub fn remove_tree(&mut self, tree: String) {
    if !self.tree_map.contains_key(&tree) {
      println!("tree does not exist: {}", tree);
    }

    if tree.starts_with(self.metadata_path.as_str()) {
      println!("metadata trees are forbidden: {}", tree);
      return
    }
    
    self.tree_map.remove(&tree);
    self.db.drop_tree(tree.to_owned()).expect(format!("drop_tree failed for {}", tree).as_str());
  }

  pub fn get_tree(&self, tree: String) -> Option<&sled::Tree> {
    self.tree_map.get(&tree)
  }
}