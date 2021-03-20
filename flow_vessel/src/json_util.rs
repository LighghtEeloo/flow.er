use serde::{Deserialize, Serialize};
use serde_json::from_str as from_json_str;
use serde_json::json;
use serde_json::to_string as to_json_string;
/// Basic from_json ( Json -> T ).
pub fn from_json<'a, T: Deserialize<'a>>(s: &'a str) -> T {
    from_json_str(s).expect("json deserialization failed.")
}
/// Basic to_json ( T -> Json ).
pub fn to_json<T: Serialize>(v: &T) -> String {
    to_json_string(v).expect("json serialization failed.")
}
/// Formatted to_json ( T -> Json ).
pub fn export_json<T: Serialize>(v: &T) -> String {
    let obj = json!(v);
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser).unwrap();
    let res = String::from_utf8(ser.into_inner()).unwrap();
    res
}
