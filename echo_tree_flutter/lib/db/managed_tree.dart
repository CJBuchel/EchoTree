import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:hive/hive.dart';

class ManagedTree {
  Box? _box;
  final String _treeName;
  int checksum = 0; // checksum of the tree (server side)

  ManagedTree(String treeName) : _treeName = treeName {
    if (treeName.contains('/')) {
      throw Exception("Invalid tree name: $treeName, cannot contain '/', use ':' instead.");
    }
  }

  Future<void> open() async {
    debugPrint("opening tree: $_treeName...");
    _box = await Hive.openBox(_treeName);
  }

  Future<void> insert(String key, String value) async {
    if (_box != null) {
      await _box!.put(key, value);
    } else {
      debugPrint("box is null, try opening it first: $_treeName...");
    }
  }

  String get(String key) {
    if (_box != null) {
      return _box!.get(key);
    }
    return '';
  }

  Future<void> remove(String key) async {
    if (_box != null) {
      await _box!.delete(key);
    }
  }

  Future<int> clear() async {
    if (_box != null) {
      return await _box!.clear();
    } else {
      return 0;
    }
  }

  Future<void> drop() async {
    clear();
    if (_box != null) {
      _box!.deleteFromDisk();
    }
  }

  Future<void> setFromHashmap(Map<String, String> map) async {
    if (_box != null) {
      await clear();
      List<Future> futures = [];
      map.forEach((key, value) {
        futures.add(insert(key, value));
      });
      await Future.wait(futures);
    }
  }

  Map<String, String> getAsHashmap() {
    Map<String, String> map = {};
    if (_box != null) {
      _box!.toMap().forEach((key, value) {
        map[key] = value;
      });
    }
    return map;
  }

  void forEach(void Function(String, String) f) {
    if (_box != null) {
      _box!.toMap().forEach((key, value) {
        f(key, value);
      });
    }
  }

  get getAsJson => jsonEncode(getAsHashmap());
  get getName => _treeName;
  set setChecksum(int c) => checksum = c;
}
