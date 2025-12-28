/*
 Compatibility types for Typeshare limitations
 These types are used for untagged enums that Typeshare cannot generate
 */

import Foundation

// MARK: - BTreeMap Compatibility

/// Typealias for Rust BTreeMap compatibility
public typealias BTreeMap<K: Hashable, V> = [K: V]

// MARK: - Untagged Enum Type Aliases

// These types are serialized as flexible JSON values
// because Typeshare doesn't support Rust's #[serde(untagged)] enums

public typealias SaintDateDef = AnyCodable
public typealias SaintDate = AnyCodable
public typealias SaintCount = AnyCodable
public typealias EntityRef = AnyCodable
public typealias TitlesDef = AnyCodable
public typealias DateDef = AnyCodable
public typealias DateDefExtended = AnyCodable
public typealias ExceptionCondition = AnyCodable
public typealias DateDefExceptions = AnyCodable
public typealias DateDefException = AnyCodable
public typealias ColorsDef = AnyCodable
public typealias CommonsDef = AnyCodable

// MARK: - AnyCodable

/// A type-erased Codable value for flexible JSON handling
/// Used for types that Typeshare cannot generate (untagged enums)
public struct AnyCodable: Codable, Equatable, Hashable, @unchecked Sendable {
    public let value: Any

    public init(_ value: Any) {
        self.value = value
    }

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if container.decodeNil() {
            self.value = NSNull()
        } else if let bool = try? container.decode(Bool.self) {
            self.value = bool
        } else if let int = try? container.decode(Int.self) {
            self.value = int
        } else if let double = try? container.decode(Double.self) {
            self.value = double
        } else if let string = try? container.decode(String.self) {
            self.value = string
        } else if let array = try? container.decode([AnyCodable].self) {
            self.value = array.map { $0.value }
        } else if let dict = try? container.decode([String: AnyCodable].self) {
            self.value = dict.mapValues { $0.value }
        } else {
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "AnyCodable cannot decode value")
        }
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch value {
        case is NSNull:
            try container.encodeNil()
        case let bool as Bool:
            try container.encode(bool)
        case let int as Int:
            try container.encode(int)
        case let double as Double:
            try container.encode(double)
        case let string as String:
            try container.encode(string)
        case let array as [Any]:
            try container.encode(array.map { AnyCodable($0) })
        case let dict as [String: Any]:
            try container.encode(dict.mapValues { AnyCodable($0) })
        default:
            throw EncodingError.invalidValue(value, EncodingError.Context(codingPath: container.codingPath, debugDescription: "AnyCodable cannot encode value"))
        }
    }

    public static func == (lhs: AnyCodable, rhs: AnyCodable) -> Bool {
        switch (lhs.value, rhs.value) {
        case is (NSNull, NSNull):
            return true
        case let (l as Bool, r as Bool):
            return l == r
        case let (l as Int, r as Int):
            return l == r
        case let (l as Double, r as Double):
            return l == r
        case let (l as String, r as String):
            return l == r
        default:
            return false
        }
    }

    public func hash(into hasher: inout Hasher) {
        switch value {
        case let bool as Bool:
            hasher.combine(bool)
        case let int as Int:
            hasher.combine(int)
        case let double as Double:
            hasher.combine(double)
        case let string as String:
            hasher.combine(string)
        default:
            hasher.combine(0)
        }
    }
}
