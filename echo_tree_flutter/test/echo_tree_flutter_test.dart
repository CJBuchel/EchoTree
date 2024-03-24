// import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:echo_tree_flutter/client/client.dart';
import 'package:flutter_test/flutter_test.dart';

// import 'package:echo_tree_flutter/echo_tree_flutter.dart';
import 'package:logger/logger.dart';

void main() async {
  Logger().i('echo_tree_flutter_test.dart');

  EchoTreeClient().connect("http://localhost:2121");

  // wait for 5 seconds
  await Future.delayed(const Duration(seconds: 5));

  // EchoTreeClient().disconnect();

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
