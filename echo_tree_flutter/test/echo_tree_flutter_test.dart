// import 'package:echo_tree_flutter/schema/schema.dart';

import 'dart:convert';

import 'package:echo_tree_flutter/client/client.dart';
import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:flutter_test/flutter_test.dart';

// import 'package:echo_tree_flutter/echo_tree_flutter.dart';
import 'package:logger/logger.dart';

void main() async {
  Logger().i('echo_tree_flutter_test.dart');

  await EchoTreeClient().connect(
    "http://localhost:2121",
  );
  EchoTreeClient().subscribe(["test:user"]);

  Logger().i("Sending test in 2...");
  await Future.delayed(const Duration(seconds: 2));

  await EchoTreeClient().authenticate("public", "public");

  var clientTest = TestStruct(test: "UwU from client").toJson();

  Logger().i("Client Send");
  EchoTreeClient().insert("test:user", "Client", jsonEncode(clientTest));

  // wait for 5 seconds
  await Future.delayed(const Duration(seconds: 20));

  await EchoTreeClient().disconnect();

  test('Test', () {});
}
