import 'dart:convert';

import 'package:echo_tree_flutter/db/db.dart';
import 'package:echo_tree_flutter/schema/schema.dart';

class EchoItemBroker {
  static final EchoItemBroker _instance = EchoItemBroker._internal();
  factory EchoItemBroker() {
    return _instance;
  }

  EchoItemBroker._internal();

  void _remove(String treeName, int checksum, String key) {
    // delete the item
    Database().getTreeMap?.getTree(treeName).remove(key).then((_) {
      // update checksum
      Database().getTreeMap?.getTree(treeName).setChecksum = checksum;
    });
  }

  void _insert(String treeName, int checksum, String key, String data) {
    // insert the item
    Database().getTreeMap?.getTree(treeName).insert(key, data).then((_) {
      // update checksum
      Database().getTreeMap?.getTree(treeName).setChecksum = checksum;
    });
  }

  // broker method
  void broker(String message) {
    EchoItemEvent event = EchoItemEvent.fromJson(jsonDecode(message));

    // convert to echo item event
    if (event.data.isEmpty) {
      _remove(event.treeName, event.checksum, event.key);
    } else {
      // update the item
      _insert(event.treeName, event.checksum, event.key, event.data);
    }
  }
}
