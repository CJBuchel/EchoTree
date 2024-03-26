// To parse this JSON data, do
//
//     final schema = schemaFromJson(jsonString);

import 'dart:convert';

Schema schemaFromJson(String str) => Schema.fromJson(json.decode(str));

String schemaToJson(Schema data) => json.encode(data.toJson());

class Schema {
    ChecksumEvent checksumEvent;
    DeleteEvent deleteEvent;
    EchoItemEvent echoItemEvent;
    EchoTreeClientSocketMessage echoTreeClientSocketMessage;
    EchoTreeEvent echoTreeEvent;
    EchoTreeServerSocketMessage echoTreeServerSocketMessage;
    GetEvent getEvent;
    GetTreeEvent getTreeEvent;
    RegisterRequest registerRequest;
    RegisterResponse registerResponse;
    StatusResponseEvent responseEvent;
    Role role;
    RoleAuthenticateRequest roleAuthenticateRequest;
    InsertEvent setEvent;
    SetTreeEvent setTreeEvent;
    SubscribeEvent subscribeEvent;
    TestStruct testStruct;
    UnsubscribeEvent unsubscribeEvent;

    Schema({
        required this.checksumEvent,
        required this.deleteEvent,
        required this.echoItemEvent,
        required this.echoTreeClientSocketMessage,
        required this.echoTreeEvent,
        required this.echoTreeServerSocketMessage,
        required this.getEvent,
        required this.getTreeEvent,
        required this.registerRequest,
        required this.registerResponse,
        required this.responseEvent,
        required this.role,
        required this.roleAuthenticateRequest,
        required this.setEvent,
        required this.setTreeEvent,
        required this.subscribeEvent,
        required this.testStruct,
        required this.unsubscribeEvent,
    });

    factory Schema.fromJson(Map<String, dynamic> json) => Schema(
        checksumEvent: ChecksumEvent.fromJson(json["checksum_event"]),
        deleteEvent: DeleteEvent.fromJson(json["delete_event"]),
        echoItemEvent: EchoItemEvent.fromJson(json["echo_item_event"]),
        echoTreeClientSocketMessage: EchoTreeClientSocketMessage.fromJson(json["echo_tree_client_socket_message"]),
        echoTreeEvent: EchoTreeEvent.fromJson(json["echo_tree_event"]),
        echoTreeServerSocketMessage: EchoTreeServerSocketMessage.fromJson(json["echo_tree_server_socket_message"]),
        getEvent: GetEvent.fromJson(json["get_event"]),
        getTreeEvent: GetTreeEvent.fromJson(json["get_tree_event"]),
        registerRequest: RegisterRequest.fromJson(json["register_request"]),
        registerResponse: RegisterResponse.fromJson(json["register_response"]),
        responseEvent: StatusResponseEvent.fromJson(json["response_event"]),
        role: Role.fromJson(json["role"]),
        roleAuthenticateRequest: RoleAuthenticateRequest.fromJson(json["role_authenticate_request"]),
        setEvent: InsertEvent.fromJson(json["set_event"]),
        setTreeEvent: SetTreeEvent.fromJson(json["set_tree_event"]),
        subscribeEvent: SubscribeEvent.fromJson(json["subscribe_event"]),
        testStruct: TestStruct.fromJson(json["test_struct"]),
        unsubscribeEvent: UnsubscribeEvent.fromJson(json["unsubscribe_event"]),
    );

    Map<String, dynamic> toJson() => {
        "checksum_event": checksumEvent.toJson(),
        "delete_event": deleteEvent.toJson(),
        "echo_item_event": echoItemEvent.toJson(),
        "echo_tree_client_socket_message": echoTreeClientSocketMessage.toJson(),
        "echo_tree_event": echoTreeEvent.toJson(),
        "echo_tree_server_socket_message": echoTreeServerSocketMessage.toJson(),
        "get_event": getEvent.toJson(),
        "get_tree_event": getTreeEvent.toJson(),
        "register_request": registerRequest.toJson(),
        "register_response": registerResponse.toJson(),
        "response_event": responseEvent.toJson(),
        "role": role.toJson(),
        "role_authenticate_request": roleAuthenticateRequest.toJson(),
        "set_event": setEvent.toJson(),
        "set_tree_event": setTreeEvent.toJson(),
        "subscribe_event": subscribeEvent.toJson(),
        "test_struct": testStruct.toJson(),
        "unsubscribe_event": unsubscribeEvent.toJson(),
    };
}

class ChecksumEvent {
    Map<String, int> treeChecksums;

    ChecksumEvent({
        required this.treeChecksums,
    });

    factory ChecksumEvent.fromJson(Map<String, dynamic> json) => ChecksumEvent(
        treeChecksums: Map.from(json["tree_checksums"]).map((k, v) => MapEntry<String, int>(k, v)),
    );

    Map<String, dynamic> toJson() => {
        "tree_checksums": Map.from(treeChecksums).map((k, v) => MapEntry<String, dynamic>(k, v)),
    };
}

class DeleteEvent {
    Map<String, String> treeItems;

    DeleteEvent({
        required this.treeItems,
    });

    factory DeleteEvent.fromJson(Map<String, dynamic> json) => DeleteEvent(
        treeItems: Map.from(json["tree_items"]).map((k, v) => MapEntry<String, String>(k, v)),
    );

    Map<String, dynamic> toJson() => {
        "tree_items": Map.from(treeItems).map((k, v) => MapEntry<String, dynamic>(k, v)),
    };
}

class EchoItemEvent {
    String data;
    String key;
    String treeName;

    EchoItemEvent({
        required this.data,
        required this.key,
        required this.treeName,
    });

    factory EchoItemEvent.fromJson(Map<String, dynamic> json) => EchoItemEvent(
        data: json["data"],
        key: json["key"],
        treeName: json["tree_name"],
    );

    Map<String, dynamic> toJson() => {
        "data": data,
        "key": key,
        "tree_name": treeName,
    };
}


///Echo Tree Client Socket Message message to be sent to the server (json data, represented
///by the event type)
class EchoTreeClientSocketMessage {
    String authToken;
    String? message;
    EchoTreeClientSocketEvent messageEvent;

    EchoTreeClientSocketMessage({
        required this.authToken,
        this.message,
        required this.messageEvent,
    });

    factory EchoTreeClientSocketMessage.fromJson(Map<String, dynamic> json) => EchoTreeClientSocketMessage(
        authToken: json["auth_token"],
        message: json["message"],
        messageEvent: echoTreeClientSocketEventValues.map[json["message_event"]]!,
    );

    Map<String, dynamic> toJson() => {
        "auth_token": authToken,
        "message": message,
        "message_event": echoTreeClientSocketEventValues.reverse[messageEvent],
    };
}


///Echo Tree Client Socket Event dictates the message structure, i.e: - PingEvent: (no
///message) - ChecksumEvent: tree names, checksums - SetEvent: tree, key, data - GetEvent:
///tree, key etc...
enum EchoTreeClientSocketEvent {
    CHECKSUM_EVENT,
    DELETE_EVENT,
    GET_EVENT,
    GET_TREE_EVENT,
    INSERT_EVENT,
    SET_TREE_EVENT,
    SUBSCRIBE_EVENT,
    UNSUBSCRIBE_EVENT
}

final echoTreeClientSocketEventValues = EnumValues({
    "ChecksumEvent": EchoTreeClientSocketEvent.CHECKSUM_EVENT,
    "DeleteEvent": EchoTreeClientSocketEvent.DELETE_EVENT,
    "GetEvent": EchoTreeClientSocketEvent.GET_EVENT,
    "GetTreeEvent": EchoTreeClientSocketEvent.GET_TREE_EVENT,
    "InsertEvent": EchoTreeClientSocketEvent.INSERT_EVENT,
    "SetTreeEvent": EchoTreeClientSocketEvent.SET_TREE_EVENT,
    "SubscribeEvent": EchoTreeClientSocketEvent.SUBSCRIBE_EVENT,
    "UnsubscribeEvent": EchoTreeClientSocketEvent.UNSUBSCRIBE_EVENT
});

class EchoTreeEvent {
    List<EchoTreeEventTree> trees;

    EchoTreeEvent({
        required this.trees,
    });

    factory EchoTreeEvent.fromJson(Map<String, dynamic> json) => EchoTreeEvent(
        trees: List<EchoTreeEventTree>.from(json["trees"].map((x) => EchoTreeEventTree.fromJson(x))),
    );

    Map<String, dynamic> toJson() => {
        "trees": List<dynamic>.from(trees.map((x) => x.toJson())),
    };
}

class EchoTreeEventTree {
    Map<String, String> tree;
    String treeName;

    EchoTreeEventTree({
        required this.tree,
        required this.treeName,
    });

    factory EchoTreeEventTree.fromJson(Map<String, dynamic> json) => EchoTreeEventTree(
        tree: Map.from(json["tree"]).map((k, v) => MapEntry<String, String>(k, v)),
        treeName: json["tree_name"],
    );

    Map<String, dynamic> toJson() => {
        "tree": Map.from(tree).map((k, v) => MapEntry<String, dynamic>(k, v)),
        "tree_name": treeName,
    };
}


///Echo Tree Server Socket Message message to be sent to the client (json data, represented
///by the event type)
class EchoTreeServerSocketMessage {
    String authToken;
    String? message;
    EchoTreeServerSocketEvent messageEvent;

    EchoTreeServerSocketMessage({
        required this.authToken,
        this.message,
        required this.messageEvent,
    });

    factory EchoTreeServerSocketMessage.fromJson(Map<String, dynamic> json) => EchoTreeServerSocketMessage(
        authToken: json["auth_token"],
        message: json["message"],
        messageEvent: echoTreeServerSocketEventValues.map[json["message_event"]]!,
    );

    Map<String, dynamic> toJson() => {
        "auth_token": authToken,
        "message": message,
        "message_event": echoTreeServerSocketEventValues.reverse[messageEvent],
    };
}


///Echo Tree Event dictates the message structure, i.e: - PingEvent: (no message) -
///EchoTreeEvent: trees, data - EchoItemEvent: tree, key, data etc...
enum EchoTreeServerSocketEvent {
    ECHO_ITEM_EVENT,
    ECHO_TREE_EVENT,
    STATUS_RESPONSE_EVENT
}

final echoTreeServerSocketEventValues = EnumValues({
    "EchoItemEvent": EchoTreeServerSocketEvent.ECHO_ITEM_EVENT,
    "EchoTreeEvent": EchoTreeServerSocketEvent.ECHO_TREE_EVENT,
    "StatusResponseEvent": EchoTreeServerSocketEvent.STATUS_RESPONSE_EVENT
});

class GetEvent {
    String key;
    String treeName;

    GetEvent({
        required this.key,
        required this.treeName,
    });

    factory GetEvent.fromJson(Map<String, dynamic> json) => GetEvent(
        key: json["key"],
        treeName: json["tree_name"],
    );

    Map<String, dynamic> toJson() => {
        "key": key,
        "tree_name": treeName,
    };
}

class GetTreeEvent {
    List<String> treeNames;

    GetTreeEvent({
        required this.treeNames,
    });

    factory GetTreeEvent.fromJson(Map<String, dynamic> json) => GetTreeEvent(
        treeNames: List<String>.from(json["tree_names"].map((x) => x)),
    );

    Map<String, dynamic> toJson() => {
        "tree_names": List<dynamic>.from(treeNames.map((x) => x)),
    };
}

class RegisterRequest {
    List<String> echoTrees;
    String? password;
    String? roleId;

    RegisterRequest({
        required this.echoTrees,
        this.password,
        this.roleId,
    });

    factory RegisterRequest.fromJson(Map<String, dynamic> json) => RegisterRequest(
        echoTrees: List<String>.from(json["echo_trees"].map((x) => x)),
        password: json["password"],
        roleId: json["role_id"],
    );

    Map<String, dynamic> toJson() => {
        "echo_trees": List<dynamic>.from(echoTrees.map((x) => x)),
        "password": password,
        "role_id": roleId,
    };
}

class RegisterResponse {
    String authToken;
    Map<String, String> hierarchy;
    String url;
    String uuid;

    RegisterResponse({
        required this.authToken,
        required this.hierarchy,
        required this.url,
        required this.uuid,
    });

    factory RegisterResponse.fromJson(Map<String, dynamic> json) => RegisterResponse(
        authToken: json["auth_token"],
        hierarchy: Map.from(json["hierarchy"]).map((k, v) => MapEntry<String, String>(k, v)),
        url: json["url"],
        uuid: json["uuid"],
    );

    Map<String, dynamic> toJson() => {
        "auth_token": authToken,
        "hierarchy": Map.from(hierarchy).map((k, v) => MapEntry<String, dynamic>(k, v)),
        "url": url,
        "uuid": uuid,
    };
}

class StatusResponseEvent {
    EchoTreeClientSocketEvent? fromEvent;
    String? message;
    int statusCode;

    StatusResponseEvent({
        this.fromEvent,
        this.message,
        required this.statusCode,
    });

    factory StatusResponseEvent.fromJson(Map<String, dynamic> json) => StatusResponseEvent(
        fromEvent: echoTreeClientSocketEventValues.map[json["from_event"]]!,
        message: json["message"],
        statusCode: json["status_code"],
    );

    Map<String, dynamic> toJson() => {
        "from_event": echoTreeClientSocketEventValues.reverse[fromEvent],
        "message": message,
        "status_code": statusCode,
    };
}


///Role used for authentication to branches of the database
class Role {
    List<String> echoTrees;
    String password;
    String roleId;

    Role({
        required this.echoTrees,
        required this.password,
        required this.roleId,
    });

    factory Role.fromJson(Map<String, dynamic> json) => Role(
        echoTrees: List<String>.from(json["echo_trees"].map((x) => x)),
        password: json["password"],
        roleId: json["role_id"],
    );

    Map<String, dynamic> toJson() => {
        "echo_trees": List<dynamic>.from(echoTrees.map((x) => x)),
        "password": password,
        "role_id": roleId,
    };
}

class RoleAuthenticateRequest {
    String password;
    String roleId;

    RoleAuthenticateRequest({
        required this.password,
        required this.roleId,
    });

    factory RoleAuthenticateRequest.fromJson(Map<String, dynamic> json) => RoleAuthenticateRequest(
        password: json["password"],
        roleId: json["role_id"],
    );

    Map<String, dynamic> toJson() => {
        "password": password,
        "role_id": roleId,
    };
}

class InsertEvent {
    String data;
    String key;
    String treeName;

    InsertEvent({
        required this.data,
        required this.key,
        required this.treeName,
    });

    factory InsertEvent.fromJson(Map<String, dynamic> json) => InsertEvent(
        data: json["data"],
        key: json["key"],
        treeName: json["tree_name"],
    );

    Map<String, dynamic> toJson() => {
        "data": data,
        "key": key,
        "tree_name": treeName,
    };
}

class SetTreeEvent {
    Map<String, Map<String, String>> trees;

    SetTreeEvent({
        required this.trees,
    });

    factory SetTreeEvent.fromJson(Map<String, dynamic> json) => SetTreeEvent(
        trees: Map.from(json["trees"]).map((k, v) => MapEntry<String, Map<String, String>>(k, Map.from(v).map((k, v) => MapEntry<String, String>(k, v)))),
    );

    Map<String, dynamic> toJson() => {
        "trees": Map.from(trees).map((k, v) => MapEntry<String, dynamic>(k, Map.from(v).map((k, v) => MapEntry<String, dynamic>(k, v)))),
    };
}

class SubscribeEvent {
    List<String> treeNames;

    SubscribeEvent({
        required this.treeNames,
    });

    factory SubscribeEvent.fromJson(Map<String, dynamic> json) => SubscribeEvent(
        treeNames: List<String>.from(json["tree_names"].map((x) => x)),
    );

    Map<String, dynamic> toJson() => {
        "tree_names": List<dynamic>.from(treeNames.map((x) => x)),
    };
}

class TestStruct {
    String test;

    TestStruct({
        required this.test,
    });

    factory TestStruct.fromJson(Map<String, dynamic> json) => TestStruct(
        test: json["test"],
    );

    Map<String, dynamic> toJson() => {
        "test": test,
    };
}

class UnsubscribeEvent {
    List<String> treeNames;

    UnsubscribeEvent({
        required this.treeNames,
    });

    factory UnsubscribeEvent.fromJson(Map<String, dynamic> json) => UnsubscribeEvent(
        treeNames: List<String>.from(json["tree_names"].map((x) => x)),
    );

    Map<String, dynamic> toJson() => {
        "tree_names": List<dynamic>.from(treeNames.map((x) => x)),
    };
}

class EnumValues<T> {
    Map<String, T> map;
    late Map<T, String> reverseMap;

    EnumValues(this.map);

    Map<T, String> get reverse {
        reverseMap = map.map((k, v) => MapEntry(v, k));
        return reverseMap;
    }
}
