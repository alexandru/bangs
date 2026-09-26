use super::*;

#[test]
fn encode_keeps_uri_component_safe_characters() {
    let safe = "AZaz09-_.!~*'()";
    assert_eq!(encode_uri_component(safe), safe);
}

#[test]
fn encode_percent_encodes_the_rest() {
    assert_eq!(encode_uri_component("hello world"), "hello%20world");
    assert_eq!(encode_uri_component("a+b/c"), "a%2Bb%2Fc");
    assert_eq!(encode_uri_component("100%"), "100%25");
    assert_eq!(encode_uri_component("é"), "%C3%A9");
    assert_eq!(encode_uri_component("…"), "%E2%80%A6");
}

#[test]
fn decode_reverses_encoding() {
    assert_eq!(
        decode_uri_component("hello%20world"),
        Some("hello world".to_string())
    );
    assert_eq!(decode_uri_component("%C3%A9"), Some("é".to_string()));
    // '+' is a literal here; the space mapping happens in
    // `decode_query_value`.
    assert_eq!(decode_uri_component("a+b"), Some("a+b".to_string()));
    assert_eq!(decode_uri_component(""), Some(String::new()));
}

#[test]
fn decode_rejects_malformed_input() {
    assert_eq!(decode_uri_component("100%"), None);
    assert_eq!(decode_uri_component("%zz"), None);
    assert_eq!(decode_uri_component("%C3"), None); // truncated UTF-8
    assert_eq!(decode_uri_component("%ED%A0%BD"), None); // lone surrogate
}

#[test]
fn find_query_param_scans_search_and_hash_strings() {
    assert_eq!(find_query_param("?q=hello+world", "q"), Some("hello+world"));
    assert_eq!(find_query_param("?a=1&q=2", "q"), Some("2"));
    assert_eq!(find_query_param("#q=x%20y", "q"), Some("x%20y"));
    assert_eq!(find_query_param("?qq=1&q=2", "q"), Some("2"));
    assert_eq!(find_query_param("?q=a#b", "q"), Some("a#b"));
    assert_eq!(find_query_param("?q=", "q"), Some(""));
    assert_eq!(find_query_param("?debug", "q"), None);
    assert_eq!(find_query_param("", "q"), None);
    assert_eq!(find_query_param("?defaultBang=g", "defaultBang"), Some("g"));
}

#[test]
fn decode_query_value_postprocesses_like_the_original() {
    assert_eq!(
        decode_query_value("hello+world"),
        Some("hello world".to_string())
    );
    assert_eq!(decode_query_value("%C3%A9"), Some("é".to_string()));
    assert_eq!(decode_query_value("  x  "), Some("x".to_string()));
    assert_eq!(decode_query_value(""), None);
    assert_eq!(decode_query_value("+"), None); // decodes to a space, trims to nothing
    assert_eq!(decode_query_value("%zz"), None);
}
