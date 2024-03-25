// import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:echo_tree_flutter/client/client.dart';
import 'package:echo_tree_flutter/db/db.dart';
import 'package:flutter_test/flutter_test.dart';

// import 'package:echo_tree_flutter/echo_tree_flutter.dart';
import 'package:logger/logger.dart';

void main() async {
  Logger().i('echo_tree_flutter_test.dart');

  await EchoTreeClient().connect(
    "http://localhost:2121",
    roleId: "public",
    password: "public",
  );
  EchoTreeClient().subscribe(["test:user"]);

  await Future.delayed(const Duration(seconds: 2));

  // wait for 5 seconds
  await Future.delayed(const Duration(seconds: 30));

  Database().getTreeMap?.getTree("test:user").getAsHashmap.forEach((key, value) {
    Logger().i("Key: $key, Value: $value");
  });
  EchoTreeClient().unsubscribe(["test:user"]);

  test('Test', () {});
}
