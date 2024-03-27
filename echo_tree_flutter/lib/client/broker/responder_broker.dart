import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:echo_tree_flutter/schema/schema.dart';
import 'package:flutter/widgets.dart';

class EchoResponderBroker {
  static final EchoResponderBroker _instance = EchoResponderBroker._internal();
  final _eventController = StreamController<StatusResponseEvent>.broadcast();

  Stream<StatusResponseEvent> get eventStream => _eventController.stream;

  factory EchoResponderBroker() {
    return _instance;
  }

  EchoResponderBroker._internal();

  void broker(String message) {
    try {
      StatusResponseEvent event = StatusResponseEvent.fromJson(jsonDecode(message));
      _eventController.add(event);

      switch (event.statusCode) {
        case HttpStatus.ok:
          // debugPrint('Status OK: ${event.message}');
          break;
        case HttpStatus.badRequest:
          debugPrint('Status Bad Request: ${event.message}');
          break;
        case HttpStatus.unauthorized:
          debugPrint('Status Unauthorized: ${event.message}');
          break;
        case HttpStatus.forbidden:
          debugPrint('Status Forbidden: ${event.message}');
          break;
        case HttpStatus.notFound:
          debugPrint('Status Not Found: ${event.message}');
          break;
        case HttpStatus.internalServerError:
          debugPrint('Status Internal Server Error: ${event.message}');
          break;
        default:
          debugPrint('Status: ${event.statusCode}, Message: ${event.message}');
      }
    } catch (e) {
      debugPrint('Error: $e');
    }
  }

  void dispose() {
    _eventController.close();
  }
}

mixin EchoResponderSubscriber<T extends StatefulWidget> on State<T> {
  final _subscriptions = <StreamSubscription>[];

  void onResponse(StatusResponseEvent e, void Function(int, String) onEvent) {
    _subscriptions.add(EchoResponderBroker().eventStream.listen((event) {
      if (event.fromEvent == e.fromEvent) {
        onEvent(event.statusCode, event.message ?? "");
      }
    }));
  }

  void onAnyResponse(void Function(int, String) onEvent) {
    _subscriptions.add(EchoResponderBroker().eventStream.listen((event) {
      onEvent(event.statusCode, event.message ?? "");
    }));
  }

  void onNullFromEventResponse(void Function(int, String) onEvent) {
    _subscriptions.add(EchoResponderBroker().eventStream.listen((event) {
      if (event.fromEvent == null) {
        onEvent(event.statusCode, event.message ?? "");
      }
    }));
  }

  @override
  void dispose() {
    for (var subscription in _subscriptions) {
      subscription.cancel();
    }
    _subscriptions.clear();
    super.dispose();
  }
}
