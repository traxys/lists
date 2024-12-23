//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.12

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of openapi.api;

class OkCreateListResponse {
  /// Returns a new [OkCreateListResponse] instance.
  OkCreateListResponse({
    required this.ok,
  });

  CreateListResponse ok;

  @override
  bool operator ==(Object other) => identical(this, other) || other is OkCreateListResponse &&
     other.ok == ok;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (ok.hashCode);

  @override
  String toString() => 'OkCreateListResponse[ok=$ok]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'ok'] = this.ok;
    return json;
  }

  /// Returns a new [OkCreateListResponse] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static OkCreateListResponse? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "OkCreateListResponse[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "OkCreateListResponse[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return OkCreateListResponse(
        ok: CreateListResponse.fromJson(json[r'ok'])!,
      );
    }
    return null;
  }

  static List<OkCreateListResponse>? listFromJson(dynamic json, {bool growable = false,}) {
    final result = <OkCreateListResponse>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = OkCreateListResponse.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, OkCreateListResponse> mapFromJson(dynamic json) {
    final map = <String, OkCreateListResponse>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = OkCreateListResponse.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of OkCreateListResponse-objects as value to a dart map
  static Map<String, List<OkCreateListResponse>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<OkCreateListResponse>>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = OkCreateListResponse.listFromJson(entry.value, growable: growable,);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'ok',
  };
}

