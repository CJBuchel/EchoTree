use std::collections::HashMap;

use log::{error, warn};


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
      warn!("tree already exists: {}", tree);
      return
    }

    if tree.starts_with(self.metadata_path.as_str()) {
      warn!("metadata trees are forbidden: {}", tree);
      return
    }

    match self.db.open_tree(tree.clone()) {
      Ok(t) => self.tree_map.insert(tree, t),
      Err(e) => {
        error!("open_tree failed for {}: {}", tree, e);
        None
      }
    };
  }

  pub fn remove_tree(&mut self, tree: String) {
    if !self.tree_map.contains_key(&tree) {
      warn!("tree does not exist: {}", tree);
    }

    if tree.starts_with(self.metadata_path.as_str()) {
      warn!("metadata trees are forbidden: {}", tree);
      return
    }
    
    self.tree_map.remove(&tree);
    match self.db.drop_tree(tree.to_owned()) {
      Ok(_) => warn!("dropped tree: {}", tree),
      Err(e) => error!("drop_tree failed for {}: {}", tree, e)
    };
  }

  pub fn get_tree(&self, tree: String) -> Option<&sled::Tree> {
    self.tree_map.get(&tree)
  }
}