// import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:echo_tree_flutter/echo_tree_flutter.dart';
import 'package:logger/logger.dart';

void main() async {
  Logger().i('echo_tree_flutter_test.dart');

  EchoTreeClient client = EchoTreeClient('http://localhost:2121');

  await client.connect();

  // test echo message

  // EchoEvent echoEvent = EchoEvent(
  //   authToken: client.getAuthToken(),
  //   method: MethodType.SUBSCRIBE,
  //   params: MethodParameters(
  //     trees: ["test_from_flutter"],
  //   ),
  // );

  // client.sendMessage(echoEvent);

  test('Test', () {});
}
