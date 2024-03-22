use log::{debug, error};

use super::{role_manager::RoleManager, tree_hierarchy::TreeHierarchy, tree_map::TreeMap};

pub struct DatabaseConfig {
  db_path: String,
  metadata_path: String,
}

impl Default for DatabaseConfig {
  fn default() -> Self {
    DatabaseConfig {
      db_path: "tree.kvdb".to_string(),
      metadata_path: "metadata".to_string(),
    }
  }
}

pub struct Database {
  hierarchy: TreeHierarchy,
  trees: TreeMap,
  role_manager: RoleManager,
}

impl Database {
  pub fn new(config: DatabaseConfig) -> Database {
    let db: sled::Db = sled::open(config.db_path.clone()).expect(format!("open failed for {}", config.db_path).as_str());
    let hierarchy = TreeHierarchy::open(&db, config.metadata_path.clone());
    let role_manager = RoleManager::open(&db, config.metadata_path.clone());

    // create the tree map from the hierarchy
    let trees = hierarchy.get_tree_map();

    Database { hierarchy, trees, role_manager }
  }

  pub fn get_hierarchy(&self) -> &TreeHierarchy {
    &self.hierarchy
  }

  pub fn get_tree_map(&self) -> &TreeMap {
    &self.trees
  }

  pub fn get_tree_map_mut(&mut self) -> &mut TreeMap {
    &mut self.trees
  }

  pub fn get_role_manager(&self) -> &RoleManager {
    &self.role_manager
  }

  // clears all the values in every tree (does not delete the trees themselves)
  pub fn clear(&mut self) {
    self.trees.clear();
    self.hierarchy.clear();
    self.role_manager.clear();
  }

  // drops all the trees, not recoverable unless new hierarchy is created and new trees are opened
  pub fn drop(&mut self) {
    self.trees.drop();
    self.hierarchy.drop();
    self.role_manager.drop();
  }

  pub fn add_tree(&mut self, tree_name: String, schema: String) {
    self.hierarchy.insert_schema(tree_name.clone(), schema);
    self.trees.open_tree(tree_name);
  }

  pub fn remove_tree(&mut self, tree_name: String) {
    self.hierarchy.remove_schema(tree_name.clone());
    self.trees.remove_tree(tree_name);
  }

  // returns value if it exists
  pub fn insert(&mut self, tree_name: String, key: String, value: String) -> Option<String> {
    debug!("INSERT into tree: {}, key: {}", tree_name, key);
    match self.trees.get_tree_mut(tree_name.clone()) {
      Some(tree) => {
        match tree.insert(key.as_bytes(), value.as_bytes()) {
          // IVec as string
          Ok(v) => {
            match v {
              Some(v) => Some(std::str::from_utf8(&v).unwrap_or_default().to_string()),
              None => None
            }
          },
          Err(e) => {
            error!("insert failed: {}", e);
            None
          }
        }
      },
      None => None
    }
  }

  // returns value if it exists
  pub fn get(&self, tree_name: String, key: String) -> Option<String> {
    debug!("GET from tree: {}, key: {}", tree_name, key);
    match self.trees.get_tree(tree_name.clone()) {
      Some(tree) => {
        match tree.get(key.as_bytes()) {
          Ok(Some(value)) => Some(std::str::from_utf8(&value).unwrap().to_string()),
          Ok(None) => None,
          Err(e) => {
            error!("get failed: {}", e);
            None
          }
        }
      },
      None => None
    }
  }

  // returns value if it exists
  pub fn remove(&mut self, tree_name: String, key: String) -> Option<String> {
    debug!("REMOVE from tree: {}, key: {}", tree_name, key);
    match self.trees.get_tree_mut(tree_name.clone()) {
      Some(tree) => {
        match tree.remove(key.as_bytes()) {
          Ok(v) => {
            match v {
              Some(v) => Some(std::str::from_utf8(&v).unwrap().to_string()),
              None => None
            }
          },
          Err(e) => {
            error!("remove failed: {}", e);
            None
          }
        }
      },
      None => None
    }
  }
}