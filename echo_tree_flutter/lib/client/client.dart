import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:echo_tree_flutter/client/broker/broker.dart';
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

  // main connection flag
  bool _connected = false;
  Timer? _checksumTimer;

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

  // send checksum event to server (if connected)
  void _sendChecksumsEvent() async {
    if (!_connected) return;
    debugPrint("sending checksums...");
    final event = ChecksumEvent(treeChecksums: Database().getChecksums).toJson();
    final message = EchoTreeClientSocketMessage(
      authToken: _authToken,
      messageEvent: EchoTreeClientSocketEvent.CHECKSUM_EVENT,
      message: jsonEncode(event),
    ).toJson();

    _channel?.sink.add(jsonEncode(message));
  }

  // start the checksum timer (every 10 seconds send checksums to server)
  void _startChecksumTimer() {
    _checksumTimer = Timer.periodic(const Duration(seconds: 10), (_) {
      _sendChecksumsEvent();
    });
  }

  // reset the checksum timer, (generally done when a proper message is send from the server)
  void _resetChecksumTimer() {
    _checksumTimer?.cancel();
    _startChecksumTimer();
  }

  void _listen() async {
    try {
      _channel?.stream.listen((event) {
        try {
          final json = jsonDecode(event);
          EchoTreeServerSocketMessage message = EchoTreeServerSocketMessage.fromJson(json);
          EchoTreeMessageBroker().broker(message);

          // reset the checksum timer on a proper data change
          if (message.messageEvent == EchoTreeServerSocketEvent.ECHO_ITEM_EVENT ||
              message.messageEvent == EchoTreeServerSocketEvent.ECHO_TREE_EVENT) {
            _resetChecksumTimer(); // reset the timer on server response
          }
        } catch (e) {
          debugPrint("Failed socket json decode. Error: $e");
        }
      });
    } catch (e) {
      throw Exception('Failed to listen to server on: $_connectedUrl');
    }
  }

  Future<void> connect(
    String address, {
    String? roleId,
    String? password,
    List<String>? echoTrees,
  }) async {
    // reset values
    _connected = false;
    _address = address;
    _roleId = roleId ?? "";
    _password = password ?? "";

    // check server pulse
    final pulse = await _checkPulse(address);
    if (pulse) {
      final response = await _register(
        address,
        echoTrees: echoTrees,
        roleId: roleId,
        password: password,
      );

      // set the client properties
      _connectedUrl = response.url;
      _authToken = response.authToken;
      _uuid = response.uuid;

      // initialize the database
      if (response.hierarchy.isNotEmpty) {
        debugPrint("initializing metadata...");
        Database().init('metadata', hierarchy: response.hierarchy);
      }

      // startup the websocket
      _channel = WebSocketChannel.connect(Uri.parse(_connectedUrl));
      _channel?.ready.then((_) {
        // startup the receivers and listeners
        _connected = true;
        _listen();
        _resetChecksumTimer();
      });
    } else {
      throw Exception('Failed to connect to server');
    }
  }

  void disconnect() {
    _channel?.sink.close();
    _connected = false;
  }

  void subscribe(List<String> treeNames) {
    final event = SubscribeEvent(treeNames: treeNames).toJson();
    final message = EchoTreeClientSocketMessage(
      authToken: _authToken,
      messageEvent: EchoTreeClientSocketEvent.SUBSCRIBE_EVENT,
      message: jsonEncode(event),
    ).toJson();
    _channel?.sink.add(jsonEncode(message));
  }

  void unsubscribe(List<String> treeNames) {
    final event = UnsubscribeEvent(treeNames: treeNames).toJson();
    final message = EchoTreeClientSocketMessage(
      authToken: _authToken,
      messageEvent: EchoTreeClientSocketEvent.UNSUBSCRIBE_EVENT,
      message: jsonEncode(event),
    ).toJson();
    _channel?.sink.add(jsonEncode(message));
  }
}
