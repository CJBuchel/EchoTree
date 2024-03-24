import 'dart:async';
import 'dart:convert';

import 'package:echo_tree_flutter/db/managed_tree.dart';
import 'package:flutter/foundation.dart';

class EchoTreeNotifier<K, V> extends ChangeNotifier {
  final ManagedTree managedTree;
  final V Function(dynamic) fromJson;
  Map<K, V> items = {};

  late final StreamSubscription<Map<String, dynamic>> _updatesStream;

  EchoTreeNotifier({required this.managedTree, required this.fromJson}) {
    _populateData();
    // listen and update items
    _updatesStream = managedTree.updates.listen((update) {
      update.forEach((key, value) {
        if (value == null) {
          items.remove(key as K);
        } else {
          items[key as K] = fromJson(jsonDecode(value as String));
        }
      });

      // notify listeners when the data changes
      notifyListeners();
    });
  }

  void _populateData() {
    final rawData = managedTree.getAsHashmap;
    items = rawData.map((key, value) => MapEntry(key as K, fromJson(value)));
  }

  @override
  void dispose() {
    _updatesStream.cancel();
    super.dispose();
  }
}
