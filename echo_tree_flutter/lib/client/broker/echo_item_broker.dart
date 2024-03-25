import 'dart:convert';

import 'package:echo_tree_flutter/db/db.dart';
import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:flutter/foundation.dart';

class EchoItemBroker {
  static final EchoItemBroker _instance = EchoItemBroker._internal();
  factory EchoItemBroker() {
    return _instance;
  }

  EchoItemBroker._internal();

  Future<void> _remove(String treeName, int checksum, String key) async {
    // delete the item
    await Database().getTreeMap?.getTree(treeName).remove(key).then((_) {
      // update checksum
      Database().getTreeMap?.getTree(treeName).setChecksum = checksum;
    });
  }

  Future<void> _insert(String treeName, int checksum, String key, String data) async {
    // insert the item
    await Database().getTreeMap?.getTree(treeName).insert(key, data).then((_) {
      // update checksum
      Database().getTreeMap?.getTree(treeName).setChecksum = checksum;
    });
  }

  // broker method
  Future<void> broker(String message) async {
    try {
      // convert to echo item event
      EchoItemEvent event = EchoItemEvent.fromJson(jsonDecode(message));

      if (event.data.isEmpty) {
        await _remove(event.treeName, event.checksum, event.key);
      } else {
        // update the item
        await _insert(event.treeName, event.checksum, event.key, event.data);
      }
    } catch (e) {
      debugPrint("Error: $e");
    }
  }
}
