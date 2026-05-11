use crate::text_cacher::{create_word_map, process_text};

#[test]
fn test_create_word_map_logic() {
    let test_cases = [
        (
            "Ala ma kota",
            vec![("Ala", vec![0]), ("ma", vec![1]), ("kota", vec![2])],
        ),
        ("kota kota", vec![("kota", vec![0, 1])]),
        ("", vec![]),
    ];

    for (text, expected) in test_cases {
        let map = create_word_map(text);
        assert_eq!(map.len(), expected.len());
        for (word, indices) in expected {
            assert_eq!(map.get(word).unwrap(), &indices);
        }
    }
}

#[test]
fn test_process_text_pure() {
    let text = "hello world".to_string();
    let (returned_text, returned_map) = process_text(text);

    assert_eq!(*returned_text, "hello world");
    assert!(returned_map.contains_key("hello"));
    assert!(returned_map.contains_key("world"));
}
