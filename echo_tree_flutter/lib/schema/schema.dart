// To parse this JSON data, do
//
//     final schema = schemaFromJson(jsonString);

import 'dart:convert';

Schema schemaFromJson(String str) => Schema.fromJson(json.decode(str));

String schemaToJson(Schema data) => json.encode(data.toJson());

class Schema {
    EchoEvent echoEvent;
    RegisterRequest registerRequest;
    RegisterResponse registerResponse;
    Role role;
    RoleAuthenticateRequest roleAuthenticateRequest;

    Schema({
        required this.echoEvent,
        required this.registerRequest,
        required this.registerResponse,
        required this.role,
        required this.roleAuthenticateRequest,
    });

    factory Schema.fromJson(Map<String, dynamic> json) => Schema(
        echoEvent: EchoEvent.fromJson(json["echo_event"]),
        registerRequest: RegisterRequest.fromJson(json["register_request"]),
        registerResponse: RegisterResponse.fromJson(json["register_response"]),
        role: Role.fromJson(json["role"]),
        roleAuthenticateRequest: RoleAuthenticateRequest.fromJson(json["role_authenticate_request"]),
    );

    Map<String, dynamic> toJson() => {
        "echo_event": echoEvent.toJson(),
        "register_request": registerRequest.toJson(),
        "register_response": registerResponse.toJson(),
        "role": role.toJson(),
        "role_authenticate_request": roleAuthenticateRequest.toJson(),
    };
}

class EchoEvent {
    String authToken;
    MethodType method;
    MethodParameters params;

    EchoEvent({
        required this.authToken,
        required this.method,
        required this.params,
    });

    factory EchoEvent.fromJson(Map<String, dynamic> json) => EchoEvent(
        authToken: json["auth_token"],
        method: methodTypeValues.map[json["method"]]!,
        params: MethodParameters.fromJson(json["params"]),
    );

    Map<String, dynamic> toJson() => {
        "auth_token": authToken,
        "method": methodTypeValues.reverse[method],
        "params": params.toJson(),
    };
}

enum MethodType {
    DELETE,
    ECHO,
    GET,
    SET,
    SUBSCRIBE,
    UNSUBSCRIBE
}

final methodTypeValues = EnumValues({
    "Delete": MethodType.DELETE,
    "Echo": MethodType.ECHO,
    "Get": MethodType.GET,
    "Set": MethodType.SET,
    "Subscribe": MethodType.SUBSCRIBE,
    "Unsubscribe": MethodType.UNSUBSCRIBE
});

class MethodParameters {
    String? data;
    String? key;
    List<String>? trees;

    MethodParameters({
        this.data,
        this.key,
        this.trees,
    });

    factory MethodParameters.fromJson(Map<String, dynamic> json) => MethodParameters(
        data: json["data"],
        key: json["key"],
        trees: json["trees"] == null ? [] : List<String>.from(json["trees"]!.map((x) => x)),
    );

    Map<String, dynamic> toJson() => {
        "data": data,
        "key": key,
        "trees": trees == null ? [] : List<dynamic>.from(trees!.map((x) => x)),
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
    String url;
    String uuid;

    RegisterResponse({
        required this.authToken,
        required this.url,
        required this.uuid,
    });

    factory RegisterResponse.fromJson(Map<String, dynamic> json) => RegisterResponse(
        authToken: json["auth_token"],
        url: json["url"],
        uuid: json["uuid"],
    );

    Map<String, dynamic> toJson() => {
        "auth_token": authToken,
        "url": url,
        "uuid": uuid,
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

class EnumValues<T> {
    Map<String, T> map;
    late Map<T, String> reverseMap;

    EnumValues(this.map);

    Map<T, String> get reverse {
        reverseMap = map.map((k, v) => MapEntry(v, k));
        return reverseMap;
    }
}
