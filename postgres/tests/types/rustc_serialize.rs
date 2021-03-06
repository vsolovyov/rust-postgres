extern crate rustc_serialize;

use self::rustc_serialize::json::Json;

use types::test_type;

#[test]
fn test_json_params() {
    test_type(
        "JSON",
        &[
            (
                Some(Json::from_str("[10, 11, 12]").unwrap()),
                "'[10, 11, 12]'",
            ),
            (
                Some(Json::from_str("{\"f\": \"asd\"}").unwrap()),
                "'{\"f\": \"asd\"}'",
            ),
            (None, "NULL"),
        ],
    )
}

#[test]
fn test_jsonb_params() {
    test_type(
        "JSONB",
        &[
            (
                Some(Json::from_str("[10, 11, 12]").unwrap()),
                "'[10, 11, 12]'",
            ),
            (
                Some(Json::from_str("{\"f\": \"asd\"}").unwrap()),
                "'{\"f\": \"asd\"}'",
            ),
            (None, "NULL"),
        ],
    )
}
