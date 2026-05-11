use crate::text_cacher::process_text;

#[test]
fn test_process_text_pure() {
    let text = "hello world".to_string();
    let (returned_text, returned_map) = process_text(text);

    assert_eq!(*returned_text, "hello world");
    assert!(returned_map.contains_key("hello"));
    assert!(returned_map.contains_key("world"));
}
