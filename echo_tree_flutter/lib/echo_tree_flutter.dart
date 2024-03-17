library echo_tree_flutter;

import 'dart:convert';
import 'dart:io';

import 'package:http/http.dart' as http;
import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

Future<bool> _checkPulse(String url) async {
  // check the server pulse
  final response = await http.get(Uri.parse("$url/echo_tree/pulse"));
  if (response.statusCode == HttpStatus.ok) {
    return true;
  } else {
    return false;
  }
}

Future<RegisterResponse> _registerEchoTree(
  String url, {
  required List<String> echoTrees,
  String? roleId,
  String? password,
}) async {
  // register the client
  final request = RegisterRequest(
    echoTrees: echoTrees,
    roleId: roleId,
    password: password,
  ).toJson();

  final response = await http.post(
    Uri.parse("$url/echo_tree/register"),
    body: jsonEncode(request),
    headers: {
      "Content-Type": "application/json",
    },
  );

  if (response.statusCode == HttpStatus.ok) {
    return RegisterResponse.fromJson(jsonDecode(response.body));
  } else {
    throw Exception('Failed to register client');
  }
}

class EchoTreeClient {
  final String url;

  String? _connectedUrl;
  String? _connectedAuthToken;
  WebSocketChannel? _channel;

  EchoTreeClient(this.url);

  Future<bool> checkPulse() async {
    return await _checkPulse(url);
  }

  Future<RegisterResponse> register({
    required List<String> echoTrees,
    String? roleId,
    String? password,
  }) async {
    return await _registerEchoTree(
      url,
      echoTrees: echoTrees,
      roleId: roleId,
      password: password,
    );
  }

  Future<void> connect({
    String? roleId,
    String? password,
  }) async {
    bool pulse = await _checkPulse(url);
    if (pulse) {
      // registering the client
      RegisterResponse response = await register(echoTrees: [], roleId: roleId, password: password);

      _connectedUrl = response.url;
      _connectedAuthToken = response.authToken;

      // start the websocket connection
      _channel = WebSocketChannel.connect(
        Uri.parse(_connectedUrl ?? "ws://localhost:2121"),
      );
    } else {
      throw Exception('Failed to connect to $url');
    }
  }

  void sendMessage(EchoEvent m) {
    if (_channel != null) {
      _channel!.sink.add(jsonEncode(m.toJson()));
    }
  }

  void sendRawMessage(String m) {
    if (_channel != null) {
      _channel!.sink.add(m);
    }
  }

  String getAuthToken() {
    return _connectedAuthToken ?? "";
  }

  String getConnectedUrl() {
    return _connectedUrl ?? "";
  }
}
