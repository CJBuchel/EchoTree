import 'dart:convert';
import 'dart:io';

import 'package:echo_tree_flutter/db/db.dart';
import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'package:web_socket_channel/web_socket_channel.dart';

class EchoTreeClient {
  String _address = "http://localhost:2121";
  String _authToken = "";
  String _uuid = "";
  String _connectedUrl = "";
  String _roleId = "";
  String _password = "";
  WebSocketChannel? _channel;

  // getters
  get address => _address;
  get authToken => _authToken;
  get uuid => _uuid;
  get connectedUrl => _connectedUrl;
  get roleId => _roleId;
  get password => _password;

  static final EchoTreeClient _instance = EchoTreeClient._internal();

  factory EchoTreeClient() {
    return _instance;
  }

  EchoTreeClient._internal();

  Future<bool> _checkPulse(String address) async {
    final response = await http.get(Uri.parse("$address/echo_tree/pulse"));
    return response.statusCode == HttpStatus.ok ? true : false;
  }

  Future<RegisterResponse> _register(
    String address, {
    List<String>? echoTrees,
    String? roleId,
    String? password,
  }) async {
    // register the client
    List<String> trees = echoTrees ?? [];
    final request = RegisterRequest(echoTrees: trees, roleId: roleId, password: password).toJson();

    final response = await http.post(
      Uri.parse("$address/echo_tree/register"),
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

  void connect(
    String address, {
    String? roleId,
    String? password,
  }) async {
    _address = address;
    _roleId = roleId ?? "";
    _password = password ?? "";
    final pulse = await _checkPulse(address);
    if (pulse) {
      final response = await _register(address, echoTrees: ['echo_tree']);
      _connectedUrl = response.url;
      _authToken = response.authToken;
      _uuid = response.uuid;

      if (response.hierarchy.isNotEmpty) {
        debugPrint("initializing metadata...");
        Database().init('metadata', hierarchy: response.hierarchy);
      }

      // startup the websocket
      _channel = WebSocketChannel.connect(Uri.parse(_connectedUrl));
    } else {
      throw Exception('Failed to connect to server');
    }
  }

  void disconnect() {
    _channel?.sink.close();
  }
}
