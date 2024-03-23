// Singleton class to save data to database

import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:hive/hive.dart';

class Database {
  static final Database _instance = Database._internal();

  factory Database() {
    return _instance;
  }

  Database._internal();

  Future<String> test() async {
    if (!kIsWeb) {
      String path = Directory.current.path;
      path += '/tree.kvdb';
      Hive.init(path);
    }

    await Hive.openBox('tree');
    var tree = Hive.box('tree');
    tree.put('name', 'John Cena');

    var name = tree.get('name');
    return name;
  }
}
